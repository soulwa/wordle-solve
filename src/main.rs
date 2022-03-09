use std::str::FromStr;
use std::{
    error::Error,
    io::{self, Error as IOError, Write},
};

static FIVELETTERS: &'static str = concat!(
    "WHICH,THEIR,WOULD,THERE,COULD,ABOUT,OTHER,THESE,FIRST,AFTER,THINK,",
    "YEARS,THOSE,BEING,THREE,THERE,STILL,WHERE,MIGHT,WORLD,AGAIN,NEVER,",
    "UNDER,WHILE,HOUSE,WHERE,ABOUT,LOCAL,PLACE,GREAT,SMALL,GROUP,QUITE,",
    "PARTY,EVERY,WOMEN,OFTEN,GIVEN,MONEY,POINT,NIGHT,STATE,TAKEN,RIGHT,",
    "THING,WATER,RIGHT,LARGE,ASKED,POWER,LATER,YOUNG,SINCE,TIMES,COURT,",
    "DOING,EARLY,TODAY,USING,WORDS,CHILD,UNTIL,LEVEL,KNOWN,MAJOR,BEGAN,",
    "AREAS,AFTER,WOMAN,AMONG,CLEAR,STAFF,BLACK,WHOLE,SENSE,SEEMS,THIRD,",
    "WHITE,DEATH,SHALL,HEARD,TABLE,WHOSE,ORDER,RANGE,STUDY,TRADE,HOURS,",
    "HANDS,BASED,LEAVE,HUMAN,CASES,CLASS,VOICE,SINCE,SHORT,VALUE,PAPER,",
    "SEVEN,EIGHT,PRICE,RIGHT,UNTIL,MAKES,UNION,TERMS,SOUTH,NORTH,STAGE,",
    "COMES,BRING,WEEKS,START,SHOWN,MUSIC,MONTH,TRIED,WRONG,ISSUE,AWARD,",
    "ROYAL,MOVED,LIGHT,BASIS,FIELD,ADDED,MEANS,ROUND,HEART,ABOVE,STORY,",
    "FORCE,BOARD,STOOD,BOOKS,LEGAL,MODEL,BUILT,FINAL,CLOSE,SPACE,ALONG,",
    "TOTAL,THANK,PRIME,COSTS,TAKES,HAPPY,PARTS,SPENT,FLOOR,ROUND,ALLOW,",
    "RATES,SORRY,HOTEL,MEANT,LOWER,IDEAS,BASIC,WRITE,AWARE,STYLE,RULES,",
    "NEEDS,MAYBE,GIVES,SALES,EVENT,SOUND,READY,LINES,LOOKS,WORTH,PRESS,",
    "BLOOD,GOODS,CARRY,WROTE,GREEN,SHOWS,OFFER,FORMS,MILES,NEEDS,PLANS,",
    "EARTH,TITLE,GIRLS,MEANS,GLASS,HEAVY,SPEAK,RIVER,ABOVE,MOUTH,PIECE,",
    "STAND,EXTRA,WHOLE,OLDER,FULLY,PEACE,WANTS,TYPES,BELOW,RADIO,CIVIL,",
    "FIFTY,START,KNOWS,TREES,LEARN,TRUTH,WORKS,LIVED,SHARE,AGREE,FRONT,",
    "MEDIA,AVOID,STONE,APPLY,LIVES,LATER,CHAIR,HORSE,QUEEN,NAMES,CELLS,",
    "EARLY,VISIT,STOCK,CHIEF,DRAWN,FIRMS,BEGIN,IMAGE,VIEWS,SCALE,PLANT,",
    "SPEND,VOICE,ALONE,TRUST,ENDED,CAUSE,CRIME,UNITS,SPEED,ALONG,SPOKE,",
    "STUFF,FRONT,MATCH,BUILD,REACH,FRESH,SCENE,ITEMS,PHONE,STEPS,WATCH,",
    "FORTY,SIGHT,BANKS,CLAIM,ENJOY,USERS,VIDEO,WORSE,TRAIN,TRIAL,JOINT,",
    "DOUBT,COVER,USUAL,SMILE,SIDES,WHILE,WORKS,AHEAD,RURAL,TWICE,GAMES,",
    "FUNDS,SHAPE,LIGHT,QUIET,POUND,RAISE,OUGHT,NOTED,EQUAL,HOMES,WALLS,",
    "TALKS,OFFER,CAUSE,BREAK,SITES,QUICK,PROVE,NOTES,TRACK,BIRDS,ROUTE,",
    "LIKED,OCCUR,UNDER,ROOMS,DAILY,BELOW,EXIST,CHECK,ALONE,URBAN,YOUTH,",
    "EMPTY,LUNCH,UPPER,SHARE,DRUGS,SERVE,ENTER,WASTE,FACTS,SHOOK,FAITH,",
    "SHOPS,MORAL,HEADS,BIRTH,BROKE,ENTRY,CROWN,VITAL,HOPED,TOTAL,VISIT,",
    "TESTS,OWNER,WIDER,BROAD,DRINK,CLEAN,DOORS,HENCE,TEETH,BRAIN,BRIEF,",
    "SIGNS,COVER,CLAIM,GOALS,GUIDE,DRIVE,IDEAL,BOUND,KINDS,WORRY,MINOR,",
    "SEATS,NOISE,THICK,LOVED,METAL,GRAND,PHASE,COAST,LYING,WORST,ADULT,",
    "FACED,INDEX,SPORT,JUDGE,BROWN,FUNNY,INNER,LEAST,PAGES,SHARP,DRIVE,",
    "NAMED,SIXTY,AGENT,BADLY,PLACE,CROSS,GROWN,CROWD,ARGUE,CATCH,TEARS,",
    "ALIVE,BEGUN,YOURS,ANGRY,SHEET,MOTOR,SHOCK,CLOSE,DRESS,GRASS,FRUIT,",
    "TOWNS,LUCKY,TOUCH,PLATE,TIRED,FIGHT,SLEEP,TEAMS,STARS,CHEAP,CARDS,",
    "ROADS,GRANT,THEME,ERROR,DREAM,HELLO,CHEST,REFER,BEACH,FOCUS,CLUBS,",
    "BREAD,ADMIT,CHIEF,STEEL,LEADS,WAGES,TASKS,PANEL,YARDS,CHAIN,TELLS,",
    "ARMED,SLEEP,SHOES,DROVE,FALSE,SUGAR,BLOCK,ASIDE,STORE,BREAK,RAPID,",
    "FIXED,AIMED,OWNED,DRAMA,UNCLE,FIFTH,LINKS,SOLID,APART,SKILL,CLOSE,",
    "DEALT,FILMS,ROUND,TASTE,PLANE,SCOPE,FAULT,ENEMY,ROUGH,LIMIT,ABUSE,",
    "TOWER,ANGER,SWEET,ARISE,POINT,FACES,FEELS,COSTS,INPUT,TOUGH,SAVED,",
    "TRULY,DRINK,TURNS,TOOLS,CYCLE,NURSE,FRAME,PROUD,PILOT,CALLS,GIVEN,",
    "VOTES,CREAM,FEWER,THROW,AWFUL,THREW,HILLS,PRIZE,NOVEL,DEPTH,CALLS,",
    "BILLS,REPLY,TREAT,GREEN,SHEEP,STUDY,CRIED,SOUND,DANCE,SORTS,PRESS,",
    "FILES,FIGHT,STILL,ROCKS,PLAIN,WATCH,SINCE,FINDS,RATIO,COACH,FEARS,",
    "SMOKE,RUGBY,SONGS,CLOCK,FIXED,HELPS,CHOSE,MINDS,MARKS,TRUST,STEAM,",
    "SILLY,TEACH,UNITY,TAXES,SHIRT,ROUND,FINAL,BEING,ROLES,SCORE,LOANS,",
    "DOZEN,PRIDE,NEWLY,BUYER,MATCH,SHIPS,WAVES,KNIFE,PROOF,MARRY,LIVES,",
    "PITCH,BOXES,HOLES,ABOVE,APPLE,DIRTY,HOLDS,TREND,LOOSE,STATE,BLIND,",
    "KNEES,BOOTS,SMELL,MUMMY,KEEPS,WHEEL,TOUCH,SHIFT,DRAFT,SQUAD,FLESH,",
    "SPLIT,SIXTH,LEVEL,ROOTS,STICK,LAYER,RISKS,CURVE,ADOPT,GUARD,OUTER,",
    "TOPIC,TENDS,HOPES,ANGLE,GUESS,WINGS,CLEAR,MEALS,RULED,PLAYS,TEXTS,",
    "TIGHT,OPERA,PUPIL,BLAME,MIXED,GUEST,LOGIC,ACUTE,VOTED,ACTED,DELAY,",
    "VALID,HABIT,BONES,CROSS,URGED,STORM,GROSS,STUCK,MALES,PAINT,POSTS,",
    "EXACT,DADDY,AUDIT,LISTS,ALBUM,LEAVE,FORCE,FALLS,CANAL,FOCUS,MAYOR,",
    "WORTH,COUNT,DOUBT,CRASH,PRINT,LAUGH,PAIRS,LEASE,GAINS,FOODS,ALARM,",
    "SHEER,COUNT,CLOUD,GENES,LIKES,LANDS,SWEPT,NAKED,ASSET,BANDS,ACTOR,",
    "CRAFT,PLANS,DIARY,OCEAN,BENCH,MIXED,BOATS,JUDGE,KNOWN,BIBLE,MOVES,",
    "FIRED,CLOTH,SHELL,PIANO,CLERK,GATES,BONDS,WIVES,SOLVE,SADLY,SPARE,",
    "GRADE,STAKE,ASIAN,CHEEK,ALTER,SHAME,DATES,ABBEY,FLEET,STAND,FLATS,",
    "DEBTS,BURST,STICK,FAILS,LOCAL,CABLE,CHECK,CHIPS,ORDER,BRICK,GIANT,",
    "HOPES,FARMS,GRAIN,FRAUD,SWUNG,NASTY,MOVIE,SHOWS,FORUM,RELAX,CRAZY,",
    "SHOTS,REIGN,GUILT,LOVER,SLEPT,UPSET,FORMS,MOUSE,SIZES,VILLA,EDGES,",
    "PANIC,LABEL,THEFT,RISEN,DEVIL,GIFTS,DYING,MAGIC,BRAVE,LAUGH,OPENS,",
    "EATEN,GLORY,FENCE,JUICE,HATED,LIVER,SEEDS,MOVES,CHAOS,RANKS,ISSUE,",
    "CLEAN,TRAIN,POEMS,DRUNK,PAUSE,STRIP,SUPER,ACRES,ESSAY,AROSE,PATCH,",
    "CROPS,LIMIT,RACES,CLIMB,WIDOW,STEEP,DEBUT,CHART,WOODS,GRACE,BASES,",
    "HARSH,LORDS,FIBRE,BRASS,BALLS,FAINT,ROSES,FLUID,SEEKS,VAGUE,VIRUS,",
    "SHIFT,NAVAL,SHOOT,KINGS,WAVED,ADDED,MAGIC,SWORD,IMPLY,BLANK,SMART,",
    "TANKS,TRIES,BUSES,SHORE,LOADS,STIFF,CITED,RIGID,TRICK,MINES,DRANK,",
    "TAPES,EAGER,SKIRT,GRIEF,PARKS,PHONE,SHELF,WAIST,WASTE,SAUCE,COINS,",
    "DANCE,WINDS,MEETS,BORED,BONUS,DAILY,CRUEL,FACES,VERSE,GHOST,SHADE,",
    "FATAL,SLOPE,ANGEL,STRAW,UPSET,RIVAL,LOYAL,PATHS,SCORE,NOBLE,NAILS,",
    "LORRY,BRAND,LOOKS,ORGAN,CARED,MANOR,CRUDE,BEANS,BRUSH,SPELL,DATED,",
    "NERVE,PENCE,SERUM,AWAKE,BLOKE,FORTH,MINUS,RIDGE,POSED,PAINT,GROWS,",
    "SUITE,REACH,OZONE,REACT,DEALS,JEANS,TALES,RALLY,GRANT,STUCK,EAGLE,",
    "CHARM,GRAVE,CODES,REPLY,HUMAN,SOLAR,POLES,SHAKE,BLACK,BOWEL,PHOTO,",
    "SPOTS,KNOCK,BLUES,SOUND,LOVES,PRIOR,BREED,GUIDE,MODES,BUNCH,FIRES,",
    "STOPS,TOXIC,LEMON,BASIN,RINGS,SWING,FLOOD,TRAIL,LAKES,FETCH,BOMBS,",
    "LINED,PENNY,WALKS,VENUE,DEALS,BLOWN,TILES,FANCY,CRACK,HEELS,TRUCK,",
    "PLAYS,ALIKE,SMELL,WIPED,TRACE,USAGE,CORPS,ZONES,BACKS,PIPES,WIDTH,",
    "WHITE,SMOKE,CAMPS,GAZED,SALAD,ARRAY,MAJOR,PLAIN,TENTH,SKULL,JOKES,",
    "POOLS,TWINS,BORNE,YIELD,THUMB,DYING,CLASH,ARMED,WOUND,CABIN,MEDAL,",
    "TRIPS,MERCY,BLADE,DRAWS,STAMP,FERRY,ALPHA,FLOWN,ELBOW,CLIFF,NOVEL,",
    "SWEAT,PAINS,HONEY,WEIRD,TUTOR,PORTS,FLUNG,FEVER,TIGHT,WINES,SMILE,",
    "FINED,MARCH,POLLS,LIMBS,MOUNT,TRACE,PULSE,WRIST,ATOMS,BRIDE,REALM,",
    "CREWS,FLAME,FLOUR,PRINT,BOOST,LASER,YACHT,ARROW,VIVID,NOISY,QUOTE,",
    "GRAPH,BOOST,BURNT,CEASE,SHOUT,CHOIR,ACIDS,MAKER,TOURS,SPARE,ADAPT,",
    "CIVIC,BELLS,ALTOS,STEAL,SASSY,SSSSO,ASHES,MOOSE,EMOTE,FLIES,EXILE,",
    "EERIE,SLIDE,ERODE,LOOMS"
);

static FOURLETTERS: &'static str = concat!(
    "ABLE,ACID,AGED,ALSO,AREA,ARMY,AWAY,BABY,BACK,BALL,BAND,BANK,BASE,",
    "BATH,BEAR,BEAT,BEEN,BEER,BELL,BELT,BEST,BILL,BIRD,BLOW,BLUE,BOAT,",
    "BODY,BOMB,BOND,BONE,BOOK,BOOM,BORN,BOSS,BOTH,BOWL,BULK,BURN,BUSH,",
    "BUSY,CALL,CALM,CAME,CAMP,CARD,CARE,CASE,CASH,CAST,CELL,CHAT,CHIP,",
    "CITY,CLUB,COAL,COAT,CODE,COLD,COME,COOK,COOL,COPE,COPY,CORE,COST,",
    "CREW,CROP,DARK,DATA,DATE,DAWN,DAYS,DEAD,DEAL,DEAN,DEAR,DEBT,DEEP,",
    "DENY,DESK,DIAL,DIET,DIRT,DISC,DISK,DOES,DONE,DOOR,DOSE,DOWN,DRAW,",
    "DREW,DROP,DRUG,DUAL,DUKE,DUST,DUTY,EACH,EARN,EASE,EAST,EASY,EDGE,",
    "ELSE,EVEN,EVER,EVIL,EXIT,FACE,FACT,FAIL,FAIR,FALL,FARM,FAST,FATE,",
    "FEAR,FEED,FEEL,FEET,FELL,FELT,FILE,FILL,FILM,FIND,FINE,FIRE,FIRM,",
    "FISH,FIVE,FLAT,FLOW,FOOD,FOOT,FORD,FORM,FORT,FOUR,FREE,FROM,FUEL,",
    "FULL,FUND,GAIN,GAME,GATE,GAVE,GEAR,GENE,GIFT,GIRL,GIVE,GLAD,GOAL,",
    "GOES,GOLD,GOLF,GONE,GOOD,GRAY,GREW,GREY,GROW,GULF,HAIR,HALF,HALL,",
    "HAND,HANG,HARD,HARM,HATE,HAVE,HEAD,HEAR,HEAT,HELD,HELL,HELP,HERE,",
    "HERO,HIGH,HILL,HIRE,HOLD,HOLE,HOLY,HOME,HOPE,HOST,HOUR,HUGE,HUNG,",
    "HUNT,HURT,IDEA,INCH,INTO,IRON,ITEM,JACK,JANE,JEAN,JOHN,JOIN,JUMP,",
    "JURY,JUST,KEEN,KEEP,KENT,KEPT,KICK,KILL,KIND,KING,KNEE,KNEW,KNOW,",
    "LACK,LADY,LAID,LAKE,LAND,LANE,LAST,LATE,LEAD,LEFT,LESS,LIFE,LIFT,",
    "LIKE,LINE,LINK,LIST,LIVE,LOAD,LOAN,LOCK,LOGO,LONG,LOOK,LORD,LOSE,",
    "LOSS,LOST,LOVE,LUCK,MADE,MAIL,MAIN,MAKE,MALE,MANY,MARK,MASS,MATT,",
    "MEAL,MEAN,MEAT,MEET,MENU,MERE,MIKE,MILE,MILK,MILL,MIND,MINE,MISS,",
    "MODE,MOOD,MOON,MORE,MOST,MOVE,MUCH,MUST,NAME,NAVY,NEAR,NECK,NEED,",
    "NEWS,NEXT,NICE,NICK,NINE,NONE,NOSE,NOTE,OKAY,ONCE,ONLY,ONTO,OPEN,",
    "ORAL,OVER,PACE,PACK,PAGE,PAID,PAIN,PAIR,PALM,PARK,PART,PASS,PAST,",
    "PATH,PEAK,PICK,PINK,PIPE,PLAN,PLAY,PLOT,PLUG,PLUS,POLL,POOL,POOR,",
    "PORT,POST,PULL,PURE,PUSH,RACE,RAIL,RAIN,RANK,RARE,RATE,READ,REAL,",
    "REAR,RELY,RENT,REST,RICE,RICH,RIDE,RING,RISE,RISK,ROAD,ROCK,ROLE,",
    "ROLL,ROOF,ROOM,ROOT,ROSE,RULE,RUSH,RUTH,SAFE,SAID,SAKE,SALE,SALT,",
    "SAME,SAND,SAVE,SEAT,SEED,SEEK,SEEM,SEEN,SELF,SELL,SEND,SENT,SEPT,",
    "SHIP,SHOP,SHOT,SHOW,SHUT,SICK,SIDE,SIGN,SITE,SIZE,SKIN,SLIP,SLOW,",
    "SNOW,SOFT,SOIL,SOLD,SOLE,SOME,SONG,SOON,SORT,SOUL,SPOT,STAR,STAY,",
    "STEP,STOP,SUCH,SUIT,SURE,TAKE,TALE,TALK,TALL,TANK,TAPE,TASK,TEAM,",
    "TECH,TELL,TEND,TERM,TEST,TEXT,THAN,THAT,THEM,THEN,THEY,THIN,THIS,",
    "THUS,TILL,TIME,TINY,TOLD,TOLL,TONE,TONY,TOOK,TOOL,TOUR,TOWN,TREE,",
    "TRIP,TRUE,TUNE,TURN,TWIN,TYPE,UNIT,UPON,USED,USER,VARY,VAST,VERY,",
    "VICE,VIEW,VOTE,WAGE,WAIT,WAKE,WALK,WALL,WANT,WARD,WARM,WASH,WAVE,",
    "WAYS,WEAK,WEAR,WEEK,WELL,WENT,WERE,WEST,WHAT,WHEN,WHOM,WIDE,WIFE,",
    "WILD,WILL,WIND,WINE,WING,WIRE,WISE,WISH,WITH,WOOD,WORD,WORE,WORK,",
    "YARD,YEAH,YEAR,YOUR,ZERO,ZONE"
);

fn main() -> Result<(), Box<dyn Error>> {
    let five_words: Vec<&str> = FIVELETTERS.split(",").collect();
    let four_words: Vec<&str> = FOURLETTERS.split(",").collect();

    println!("{:?}", five_words);

    loop {
        let guess = prompt_input("input your Wordle guess (4-5 letters):", |s| {
            let guess = s.to_uppercase();
            let res = (s.len() == 4 && four_words.contains(&guess.as_str()))
                || (s.len() == 5 && five_words.contains(&guess.as_str()));
            if !res {
                println!(
                    "guess {} should be 4-5 letters, and in the dictionary provided!",
                    s
                );
            }
            res
        })?;

        let colors = prompt_input("input the Wordle colors ([G]reen/[Y]ellow/[M]iss)", |s| {
            let res = s.len() == guess.len() && s.chars().all(|c| "GYMgym".contains(c));
            if !res {
                println!("ensure that your colors is the same length of the guess, and only contains G, Y, and M!");
            }
            res
        })?;

        // convert to letter/color pairs
        let feedback = guess
            .chars()
            .zip(
                colors
                    .chars()
                    .map(|ch| ch.to_string().parse::<GuessColor>()),
            )
            .collect::<Vec<_>>();
        println!("{:?}", feedback);

        let five_words = five_words.filter(|word| {
            
        })
    }

    Ok(())
}

fn prompt_input<F>(prompt: &str, validator: F) -> Result<String, IOError>
where
    F: Fn(&str) -> bool,
{
    loop {
        let mut buf = String::new();
        println!("{}", prompt);
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buf) {
            Ok(_n) => {
                let input = buf.trim_end().to_string();
                if validator(&input) {
                    return Ok(input);
                } else {
                    continue;
                }
            }
            Err(e) => return Err(e),
        }
    }
}

fn guess_could_be_word(guess: &[(char, GuessColor)], word: &str) -> bool {
    let processed = guess.iter().zip(word.chars()).enumerate().filter(|g| {å
         GuessColor::Green == g.1.1 && word[g.0] == g.1.0
    });
}

#[derive(Copy, Clone, Debug)]
enum GuessColor {
    Miss,
    Yellow,
    Green,
}

impl FromStr for GuessColor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.to_lowercase().as_str() {
            "g" => Ok(GuessColor::Green),
            "y" => Ok(GuessColor::Yellow),
            "m" => Ok(GuessColor::Miss),
            _ => Err("Invalid string for guess color!".to_string()),
        }
    }
}
