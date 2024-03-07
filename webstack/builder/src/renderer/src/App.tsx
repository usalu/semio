import './App.scss'
import {
    useState,
    useRef,
    forwardRef,
    Ref,
    useMemo,
    Fragment,
    useImperativeHandle,
    SVGProps,
    useEffect,
    ReactElement
} from 'react'
import { KBarProvider } from 'kbar'
import { AttractionInput, Kit, PieceInput, Quality } from '@renderer/semio'

import {
    GraphView,
    type IEdge,
    type INode,
    GraphUtils,
    type SelectionT,
    IPoint,
    IGraphInput
} from 'react-digraph'
import SVG from 'react-inlinesvg'
import difference from 'lodash/difference'
import { KBarAnimator, KBarPortal, KBarPositioner, KBarSearch } from 'kbar'
import { KBarResults, useMatches } from 'kbar'
import { ActionId, ActionImpl } from 'kbar'
import {
    Avatar,
    ConfigProvider,
    GetProp,
    Modal,
    Space,
    Switch,
    Table,
    TableColumnsType,
    TableProps,
    Tag,
    Transfer,
    TransferProps,
    theme
} from 'antd'
import {
    ModalForm,
    ProForm,
    ProFormDateRangePicker,
    ProFormSelect,
    ProFormText
} from '@ant-design/pro-components'
import enUS from 'antd/lib/calendar/locale/en_US'

class SeededRandom {
    private seed: number

    constructor(seed: number) {
        this.seed = seed % 2147483647
        if (this.seed <= 0) this.seed += 2147483646
    }

    // Returns a pseudo-random number between 1 and 2^31 - 2
    next(): number {
        return (this.seed = (this.seed * 16807) % 2147483647)
    }

    // Returns a pseudo-random number between 0 (inclusive) and 1 (exclusive)
    nextFloat(): number {
        return (this.next() - 1) / 2147483646
    }

    // Returns a pseudo-random number between 0 (inclusive) and max (exclusive)
    nextInt(max: number): number {
        return Math.floor(this.nextFloat() * max)
    }
}

class Generator {
    public static generateRandomId(seed: number): string {
        const adjectives = [
            'cowardly',
            'xylographic',
            'dynamic',
            'xenophobic',
            'distant',
            'lost',
            'quick',
            'zippy',
            'trim',
            'living',
            'eager',
            'infantile',
            'worn',
            'live',
            'knowing',
            'next',
            'zesty',
            'utter',
            'rusty',
            'formal',
            'quizzical',
            'knobby',
            'little',
            'amazing',
            'vigilant',
            'primary',
            'meaty',
            'phony',
            'quixotic',
            'mute',
            'young',
            'jolly',
            'key',
            'positive',
            'miniature',
            'mild',
            'winged',
            'cheerful',
            'flashy',
            'habitual',
            'oceanic',
            'alike',
            'knotty',
            'holistic',
            'xylophonic',
            'generous',
            'quarterly',
            'yielding',
            'aquatic',
            'sad',
            'yummy',
            'melodic',
            'webbed',
            'versed',
            'zany',
            'outgoing',
            'busy',
            'unsightly',
            'bitter',
            'curly',
            'quaint',
            'nippy',
            'broken',
            'brainy',
            'grounded',
            'guarded',
            'junior',
            'crisp',
            'healthy',
            'colorless',
            'exclusive',
            'scattered',
            'zealous',
            'menacing',
            'gentle',
            'fond',
            'nifty',
            'gaping',
            'staid',
            'sordid',
            'noxious',
            'defiant',
            'rowdy',
            'nondescript',
            'personal',
            'adolescent',
            'brown',
            'dashing',
            'visible',
            'trustworthy',
            'rigid',
            'villainous',
            'regal',
            'clueless',
            'querulous',
            'dependable',
            'shiny',
            'numerous',
            'reasonable',
            'cold',
            'rebel',
            'worst',
            'broad',
            'humming',
            'runny',
            'youthful',
            'tasty',
            'beloved',
            'outlying',
            'soft',
            'belligerent',
            'determined',
            'clever',
            'upright',
            'vigorous',
            'fearless',
            'macho',
            'minor',
            'yellow',
            'greasy',
            'intent',
            'nonstop',
            'excitable',
            'yellowish',
            'xyloid',
            'gummy',
            'giving',
            'unlucky',
            'cheap',
            'gracious',
            'xerographic',
            'soulful',
            'selfish',
            'xenotropic',
            'mushy',
            'common',
            'merciful',
            'loathsome',
            'dizzy',
            'frightening',
            'guiltless',
            'kooky',
            'obscene',
            'alluring',
            'aboriginal',
            'purring',
            'makeshift',
            'klutzy',
            'fearful',
            'fitting',
            'lavish',
            'zigzag',
            'passionate',
            'fallacious',
            'pastel',
            'zonked',
            'sulky',
            'volatile',
            'gabby',
            'jazzy',
            'lamentable',
            'somber',
            'linear',
            'kind',
            'inferior',
            'identical',
            'content',
            'nappy',
            'handy',
            'erratic',
            'vagabond',
            'voluminous',
            'lowly',
            'feisty',
            'historical',
            'jaded',
            'extroverted',
            'concrete',
            'accurate',
            'joint',
            'unusual',
            'raw',
            'empty',
            'petite',
            'wooden',
            'wistful',
            'far',
            'flippant',
            'crowded',
            'aware',
            'gaudy',
            'spotless',
            'unused',
            'low',
            'rectangular',
            'shadowy',
            'spectacular',
            'tattered',
            'disgusting',
            'light',
            'dimpled',
            'grateful',
            'cooing',
            'overjoyed',
            'buttery',
            'instinctive',
            'pure',
            'xanthic',
            'forgetful',
            'quirky',
            'material',
            'jumbled',
            'heavy',
            'crazy',
            'aback',
            'nasty',
            'hanging',
            'incompetent',
            'savory',
            'indelible',
            'liquid',
            'loutish',
            'brilliant',
            'hurried',
            'open',
            'alienated',
            'imminent',
            'discrete',
            'immaculate',
            'altruistic',
            'needless',
            'notable',
            'pesky',
            'useful',
            'shrill',
            'easy',
            'abhorrent',
            'quarrelsome',
            'reddish',
            'colossal',
            'nebulous',
            'uniform',
            'scratchy',
            'labored',
            'juicy',
            'puny',
            'grumpy',
            'likely',
            'joyous',
            'necessary',
            'psychedelic',
            'buoyant',
            'woebegone',
            'mortified',
            'famous',
            'smelly',
            'defeated',
            'worried',
            'languid',
            'blind',
            'hungry',
            'literate',
            'admirable',
            'maddening',
            'tempting',
            'coordinated',
            'any',
            'rustic',
            'jealous',
            'green',
            'judicious',
            'flat',
            'exalted',
            'detailed',
            'available',
            'numb',
            'violet',
            'straight',
            'garrulous',
            'kindhearted',
            'feline',
            'ragged',
            'wretched',
            'overt',
            'tiny',
            'kosher',
            'dense',
            'immense',
            'yawning',
            'vacuous',
            'ruddy',
            'electric',
            'many',
            'deadly',
            'queasy',
            'unaware',
            'droopy',
            'treasured',
            'cynical',
            'difficult',
            'barren',
            'overwrought',
            'hellish',
            'divergent',
            'fatherly',
            'spicy',
            'ecstatic',
            'untrue',
            'panicky',
            'noisy',
            'pretty',
            'comfortable',
            'vain',
            'assorted',
            'indolent',
            'yearly',
            'noteworthy',
            'negligible',
            'psychotic',
            'frayed',
            'jagged',
            'jaunty',
            'faulty',
            'sarcastic',
            'romantic',
            'gargantuan',
            'naive',
            'strange',
            'nervous',
            'humdrum',
            'rash',
            'evanescent',
            'bowed',
            'uneven',
            'momentous',
            'tough',
            'loyal',
            'hearty',
            'monstrous',
            'encouraging',
            'aged',
            'standard',
            'futuristic',
            'snappy',
            'gusty',
            'jumpy',
            'ossified',
            'jubilant',
            'natural',
            'normal',
            'aromatic',
            'dry',
            'snobbish',
            'parched',
            'exhausted',
            'qualified',
            'bewildered',
            'limited',
            'silent',
            'prickly',
            'bronze',
            'crabby',
            'gruesome',
            'anguished',
            'draconian',
            'unconscious',
            'thorny',
            'concerned',
            'ruthless',
            'buzzing',
            'burdensome',
            'wordy',
            'frugal',
            'economic',
            'thick',
            'scholarly',
            'drafty',
            'gratis',
            'blushing',
            'candid',
            'waterlogged',
            'moldy',
            'golden',
            'turbulent',
            'gray',
            'overrated',
            'agile',
            'jobless',
            'smoggy',
            'stained',
            'genuine',
            'infinite',
            'high',
            'secret',
            'plain',
            'few',
            'mean',
            'naughty',
            'mad',
            'unwieldy',
            'verifiable',
            'total',
            'uptight',
            'periodic',
            'whimsical',
            'vapid',
            'vicious',
            'selective',
            'jittery',
            'whispering',
            'likable',
            'oily',
            'boundless',
            'oval',
            'abrasive',
            'timely',
            'slight',
            'obsolete',
            'befitting',
            'venomous',
            'striking',
            'homeless',
            'hateful',
            'gorgeous',
            'opulent',
            'perpetual',
            'picayune',
            'domineering',
            'monumental',
            'reliable',
            'erect',
            'demanding',
            'fragile',
            'public',
            'tubby',
            'verdant',
            'downright',
            'woeful',
            'deserted',
            'elementary',
            'muted',
            'bashful',
            'threatening',
            'thrifty',
            'numberless',
            'fluid',
            'sour',
            'thin',
            'powerless',
            'absent',
            'loose',
            'joyful',
            'leading',
            'basic',
            'long',
            'uncommon',
            'wan',
            'hushed',
            'obtainable',
            'past',
            'weepy',
            'valid',
            'tacky',
            'understood',
            'those',
            'large',
            'tricky',
            'prize',
            'hoarse',
            'wonderful',
            'utilized',
            'pathetic',
            'probable',
            'blue',
            'bony',
            'upbeat',
            'magenta',
            'putrid',
            'frantic',
            'blinding',
            'improbable',
            'greenish',
            'intelligent',
            'overcooked',
            'squealing',
            'velvety',
            'mere',
            'noiseless',
            'likeable',
            'sizzling',
            'thorough',
            'overdue',
            'illustrious',
            'organic',
            'fertile',
            'minty',
            'babyish',
            'rotating',
            'welcome',
            'marvelous',
            'intentional',
            'vast',
            'rough',
            'violent',
            'righteous',
            'thundering',
            'warmhearted',
            'weighty',
            'nosy',
            'dramatic',
            'beautiful',
            'quiet',
            'unwritten',
            'vibrant',
            'automatic',
            'sweltering',
            'ignorant',
            'decorous',
            'grubby',
            'oddball',
            'unbecoming',
            'defensive',
            'costly',
            'juvenile',
            'immediate',
            'illiterate',
            'great',
            'reckless',
            'crooked',
            'ordinary',
            'whole',
            'fabulous',
            'grizzled',
            'male',
            'abaft',
            'chemical',
            'laughable',
            'lacking',
            'victorious',
            'receptive',
            'flowery',
            'untried',
            'dirty',
            'educated',
            'noted',
            'ahead',
            'slushy',
            'curious',
            'traumatic',
            'piercing',
            'ubiquitous',
            'vital',
            'tiresome',
            'lucky',
            'novel',
            'peaceful',
            'blond',
            'same',
            'motherly',
            'unfit',
            'major',
            'impish',
            'vague',
            'bizarre',
            'roasted',
            'right',
            'overlooked',
            'smart',
            'curved',
            'workable',
            'playful',
            'late',
            'neighboring',
            'astonishing',
            'firm',
            'rural',
            'unfolded',
            'forsaken',
            'everlasting',
            'bawdy',
            'mealy',
            'super',
            'untidy',
            'tame',
            'unrealistic',
            'efficient',
            'new',
            'giddy',
            'plump',
            'unruly',
            'rude',
            'ablaze',
            'lazy',
            'hilarious',
            'spotty',
            'unfinished',
            'firsthand',
            'known',
            'pristine',
            'pleasant',
            'essential',
            'ambitious',
            'criminal',
            'humble',
            'honest',
            'hospitable',
            'cute',
            'excited',
            'daring',
            'lumbering',
            'colorful',
            'equal',
            'panoramic',
            'reflective',
            'stark',
            'gross',
            'ratty',
            'obedient',
            'last',
            'keen',
            'resolute',
            'enlightened',
            'needy',
            'arrogant',
            'stormy',
            'defenseless',
            'wavy',
            'ultimate',
            'real',
            'luxurious',
            'surprised',
            'childlike',
            'unselfish',
            'scandalous',
            'close',
            'soggy',
            'glum',
            'orange',
            'disturbed',
            'wee',
            'bland',
            'husky',
            'mysterious',
            'growing',
            'odd',
            'temporary',
            'flustered',
            'bubbly',
            'growling',
            'political',
            'outlandish',
            'suburban',
            'creative',
            'reflecting',
            'big',
            'clumsy',
            'damaging',
            'abundant',
            'front',
            'delayed',
            'boorish',
            'scented',
            'offbeat',
            'pricey',
            'blank',
            'amuck',
            'confused',
            'kindly',
            'pertinent',
            'vengeful',
            'exotic',
            'ornery',
            'emotional',
            'incredible',
            'sharp',
            'vivid',
            'fickle',
            'jumbo',
            'prudent',
            'stylish',
            'merry',
            'attached',
            'pessimistic',
            'unlined',
            'trained',
            'black',
            'toothsome',
            'insidious',
            'tested',
            'auspicious',
            'bulky',
            'chubby',
            'each',
            'supreme',
            'glorious',
            'capital',
            'fruitful',
            'handmade',
            'lively',
            'glittering',
            'bouncy',
            'utopian',
            'clean',
            'miscreant',
            'limping',
            'special',
            'foolhardy',
            'bold',
            'several',
            'hollow',
            'measly',
            'sparse',
            'miserly',
            'unsteady',
            'icky',
            'muddy',
            'jovial',
            'optimal',
            'ethereal',
            'hurt',
            'whopping',
            'glamorous',
            'glaring',
            'pink',
            'voiceless',
            'bruised',
            'wrong',
            'authentic',
            'abounding',
            'focused',
            'internal',
            'grotesque',
            'grey',
            'dearest',
            'impractical',
            'huge',
            'opposite',
            'delightful',
            'obese',
            'impeccable',
            'punctual',
            'weird',
            'weary',
            'virtuous',
            'poor',
            'undesirable',
            'expensive',
            'abject',
            'repentant',
            'instructive',
            'tight',
            'pointless',
            'truculent',
            'invincible',
            'better',
            'lawful',
            'helpless',
            'cagey',
            'breezy',
            'plastic',
            'bumpy',
            'diligent',
            'agonizing',
            'warlike',
            'classic',
            'luxuriant',
            'wide',
            'negative',
            'unripe',
            'voracious',
            'distinct',
            'early',
            'acrid',
            'unimportant',
            'required',
            'unhealthy',
            'embellished',
            'lovely',
            'shaggy',
            'animated',
            'glistening',
            'realistic',
            'imported',
            'back',
            'ornate',
            'shady',
            'rhetorical',
            'previous',
            'pungent',
            'admired',
            'madly',
            'safe',
            'productive',
            'gullible',
            'official',
            'thoughtful',
            'harmless',
            'crafty',
            'godly',
            'nostalgic',
            'bogus',
            'elfin',
            'earthy',
            'fast',
            'responsible',
            'baggy',
            'flagrant',
            'valuable',
            'present',
            'trashy',
            'motionless',
            'rich',
            'trifling',
            'unfortunate',
            'teeny',
            'unbiased',
            'lasting',
            'glossy',
            'gainful',
            'cavernous',
            'lonely',
            'rare',
            'wary',
            'fine',
            'earnest',
            'swanky',
            'rosy',
            'trivial',
            'expert',
            'interesting',
            'worrisome',
            'active',
            'annoyed',
            'vivacious',
            'inexpensive',
            'chilly',
            'subtle',
            'nonchalant',
            'milky',
            'cooked',
            'brave',
            'ritzy',
            'dead',
            'aspiring',
            'delirious',
            'delicate',
            'obnoxious',
            'wasteful',
            'assured',
            'incomplete',
            'barbarous',
            'careful',
            'uppity',
            'venerated',
            'dental',
            'extraneous',
            'twin',
            'wet',
            'mature',
            'feminine',
            'hysterical',
            'honorable',
            'funny',
            'ample',
            'round',
            'billowy',
            'angry',
            'palatable',
            'terrific',
            'prestigious',
            'brisk',
            'pushy',
            'average',
            'disloyal',
            'critical',
            'potable',
            'fantastic',
            'lovable',
            'stale',
            'vulgar',
            'rightful',
            'hypnotic',
            'advanced',
            'depressed',
            'hulking',
            'cool',
            'narrow',
            'obsequious',
            'hallowed',
            'wanting',
            'thoughtless',
            'cultivated',
            'unarmed',
            'grand',
            'demonic',
            'leafy',
            'gregarious',
            'entire',
            'meager',
            'chief',
            'industrious',
            'watchful',
            'variable',
            'wry',
            'sociable',
            'sudden',
            'defective',
            'adaptable',
            'fresh',
            'adhesive',
            'deafening',
            'nimble',
            'muddled',
            'nocturnal',
            'mountainous',
            'haunting',
            'hapless',
            'neat',
            'exemplary',
            'hissing',
            'fake',
            'posh',
            'painstaking',
            'humorous',
            'satisfying',
            'heavenly',
            'partial',
            'legal',
            'talented',
            'daffy',
            'clammy',
            'alleged',
            'pale',
            'windy',
            'nutritious',
            'true',
            'old',
            'doting',
            'steel',
            'profitable',
            'gaseous',
            'terrible',
            'nice',
            'certain',
            'tart',
            'acidic',
            'puzzling',
            'grim',
            'intrepid',
            'tasteless',
            'scrawny',
            'carefree',
            'innate',
            'obeisant',
            'hideous',
            'marked',
            'square',
            'precious',
            'itchy',
            'breakable',
            'other',
            'aggressive',
            'protective',
            'half',
            'aberrant',
            'only',
            'prime',
            'sick',
            'lumpy',
            'enraged',
            'gleaming',
            'nautical',
            'ultra',
            'damp',
            'mammoth',
            'beneficial',
            'grandiose',
            'disguised',
            'pleased',
            'inquisitive',
            'descriptive',
            'bewitched',
            'melted',
            'capricious',
            'maniacal',
            'moaning',
            'roomy',
            'unadvised',
            'brash',
            'wiry',
            'ten',
            'cloudy',
            'cheery',
            'berserk',
            'musty',
            'adept',
            'innocent',
            'elastic',
            'complete',
            'elegant',
            'anxious',
            'harmonious',
            'poised',
            'ideal',
            'agitated',
            'brief',
            'bare',
            'boiling',
            'eccentric',
            'academic',
            'halting',
            'faithful',
            'abortive',
            'plausible',
            'condemned',
            'limp',
            'imaginative',
            'plaintive',
            'hot',
            'forthright',
            'coherent',
            'frivolous',
            'magnificent',
            'fair',
            'tall',
            'dishonest',
            'competent',
            'disgusted',
            'virtual',
            'robust',
            'damaged',
            'decent',
            'thirsty',
            'revolving',
            'rabid',
            'crushing',
            'impossible',
            'craven',
            'pastoral',
            'inborn',
            'military',
            'calculating',
            'fanatical',
            'complicated',
            'impure',
            'sardonic',
            'rubbery',
            'level',
            'fragrant',
            'substantial',
            'immaterial',
            'dopey',
            'triangular',
            'constant',
            'grouchy',
            'showy',
            'apt',
            'stiff',
            'shocking',
            'our',
            'elderly',
            'miserable',
            'delicious',
            'used',
            'obvious',
            'ambiguous',
            'tawdry',
            'perfect',
            'stupid',
            'cumbersome',
            'acclaimed',
            'impassioned',
            'torpid',
            'even',
            'deep',
            'tender',
            'burly',
            'proud',
            'grieving',
            'therapeutic',
            'ancient',
            'macabre',
            'majestic',
            'honored',
            'thankful',
            'abusive',
            'bossy',
            'puffy',
            'frail',
            'legitimate',
            'offensive',
            'brawny',
            'dull',
            'raspy',
            'helpful',
            'sturdy',
            'distorted',
            'heartfelt',
            'misguided',
            'bleak',
            'useless',
            'fascinated',
            'first',
            'that',
            'able',
            'slim',
            'learned',
            'lush',
            'devilish',
            'bloody',
            'aching',
            'steady',
            'doubtful',
            'foolish',
            'homely',
            'racial',
            'deranged',
            'imperfect',
            'silly',
            'didactic',
            'understated',
            'bent',
            'creamy',
            'winding',
            'awesome',
            'third',
            'omniscient',
            'respectful',
            'wrathful',
            'unique',
            'illegal',
            'meek',
            'squeamish',
            'graceful',
            'scaly',
            'repulsive',
            'nutty',
            'lean',
            'frank',
            'elite',
            'wise',
            'another',
            'seemly',
            'regular',
            'serene',
            'unpleasant',
            'snotty',
            'scary',
            'irritating',
            'frosty',
            'oblong',
            'white',
            'lined',
            'lyrical',
            'ringed',
            'wild',
            'greedy',
            'dutiful',
            'granular',
            'unequaled',
            'remorseful',
            'untimely',
            'afraid',
            'unable',
            'harsh',
            'closed',
            'various',
            'telling',
            'best',
            'cuddly',
            'modern',
            'discreet',
            'rundown',
            'direful',
            'blissful',
            'axiomatic',
            'muffled',
            'euphoric',
            'feigned',
            'fumbling',
            'changeable',
            'adventurous',
            'frizzy',
            'dark',
            'hesitant',
            'gleeful',
            'tacit',
            'glass',
            'bountiful',
            'wholesale',
            'wicked',
            'embarrassed',
            'absorbing',
            'tense',
            'mixed',
            'alarming',
            'fretful',
            'handsome',
            'darling',
            'messy',
            'cut',
            'trusting',
            'tragic',
            'like',
            'unkempt',
            'thunderous',
            'tedious',
            'dependent',
            'good',
            'boring',
            'belated',
            'courageous',
            'enormous',
            'callous',
            'calm',
            'insistent',
            'monthly',
            'full',
            'weak',
            'shameless',
            'oafish',
            'warped',
            'striped',
            'delectable',
            'unwelcome',
            'lone',
            'hasty',
            'lying',
            'upset',
            'ashamed',
            'unsung',
            'cooperative',
            'shallow',
            'exciting',
            'rampant',
            'furtive',
            'petty',
            'fatal',
            'aloof',
            'threadbare',
            'forceful',
            'absolute',
            'agreeable',
            'squiggly',
            'bad',
            'faint',
            'flawless',
            'goofy',
            'tasteful',
            'annoying',
            'ugly',
            'tearful',
            'mindless',
            'familiar',
            'murky',
            'luminous',
            'classy',
            'uncovered',
            'screeching',
            'composed',
            'festive',
            'frightened',
            'mundane',
            'neighborly',
            'optimistic',
            'giant',
            'clear',
            'onerous',
            'horrible',
            'grimy',
            'witty',
            'apathetic',
            'acid',
            'flaky',
            'peppery',
            'neglected',
            'cautious',
            'observant',
            'chunky',
            'pumped',
            'frequent',
            'aggravating',
            'stable',
            'endurable',
            'taboo',
            'alarmed',
            'dear',
            'ripe',
            'flawed',
            'abstracted',
            'thinkable',
            'stimulating',
            'unhappy',
            'gigantic',
            'plush',
            'highfalutin',
            'chivalrous',
            'feeble',
            'perfumed',
            'redundant',
            'flimsy',
            'smiling',
            'polished',
            'left',
            'evasive',
            'sedate',
            'deadpan',
            'unnatural',
            'original',
            'disastrous',
            'caring',
            'cloistered',
            'sympathetic',
            'fat',
            'sandy',
            'married',
            'writhing',
            'ill',
            'short',
            'near',
            'drab',
            'infamous',
            'massive',
            'daily',
            'remarkable',
            'tired',
            'all',
            'vacant',
            'rotten',
            'mighty',
            'lewd',
            'attractive',
            'arctic',
            'united',
            'lame',
            'loud',
            'favorite',
            'adamant',
            'woozy',
            'exuberant',
            'failing',
            'plucky',
            'actual',
            'medical',
            'humiliating',
            'deficient',
            'spirited',
            'waiting',
            'guilty',
            'powerful',
            'fortunate',
            'lanky',
            'both',
            'flamboyant',
            'statuesque',
            'ethical',
            'artistic',
            'gifted',
            'elated',
            'magical',
            'malicious',
            'efficacious',
            'popular',
            'tinted',
            'drunk',
            'piquant',
            'fixed',
            'fancy',
            'ironclad',
            'speedy',
            'fluttering',
            'spontaneous',
            'worse',
            'lustrous',
            'whispered',
            'skillful',
            'female',
            'remote',
            'dusty',
            'coarse',
            'excellent',
            'enchanting',
            'experienced',
            'satisfied',
            'gamy',
            'furry',
            'hairy',
            'eatable',
            'enchanted',
            'snoopy',
            'evergreen',
            'sneaky',
            'shivering',
            'dispensable',
            'guttural',
            'devoted',
            'wobbly',
            'resonant',
            'idolized',
            'simplistic',
            'usable',
            'curvy',
            'gripping',
            'possible',
            'longing',
            'tightfisted',
            'nauseating',
            'successful',
            'exultant',
            'shy',
            'unknown',
            'spiritual',
            'which',
            'irate',
            'adorable',
            'possessive',
            'evil',
            'lethal',
            'false',
            'waggish',
            'frilly',
            'salty',
            'creepy',
            'standing',
            'unwilling',
            'hidden',
            'practical',
            'orderly',
            'alert',
            'abrupt',
            'sane',
            'informal',
            'strident',
            'faded',
            'impartial',
            'private',
            'willing',
            'talkative',
            'wacky',
            'dual',
            'profuse',
            'squeaky',
            'painful',
            'glib',
            'idealistic',
            'functional',
            'metallic',
            'stunning',
            'icy',
            'tepid',
            'direct',
            'forked',
            'subsequent',
            'tranquil',
            'steep',
            'angelic',
            'austere',
            'weekly',
            'infatuated',
            'trite',
            'freezing',
            'portly',
            'spooky',
            'perky',
            'penitent',
            'placid',
            'wiggly',
            'towering',
            'abnormal',
            'grave',
            'starchy',
            'wandering',
            'similar',
            'typical',
            'friendly',
            'bustling',
            'sticky',
            'occasional',
            'livid',
            'spiteful',
            'serpentine',
            'medium',
            'esteemed',
            'hefty',
            'tan',
            'polite',
            'rewarding',
            'tangy',
            'separate',
            'happy',
            'dim',
            'different',
            'elaborate',
            'starry',
            'physical',
            'considerate',
            'canine',
            'acoustic',
            'dimwitted',
            'worthless',
            'relieved',
            'heady',
            'recondite',
            'addicted',
            'trusty',
            'idle',
            'troubled',
            'careless',
            'masculine',
            'insecure',
            'lopsided',
            'mellow',
            'frigid',
            'sentimental',
            'fluffy',
            'hard',
            'annual',
            'loving',
            'outrageous',
            'wilted',
            'permissible',
            'sinful',
            'energetic',
            'finished',
            'abandoned',
            'sloppy',
            'disfigured',
            'red',
            'equatorial',
            'single',
            'abiding',
            'pitiful',
            'worthwhile',
            'slimy',
            'cylindrical',
            'premium',
            'groovy',
            'silky',
            'ludicrous',
            'flickering',
            'charming',
            'warm',
            'decimal',
            'svelte',
            'modest',
            'moist',
            'urban',
            'alive',
            'truthful',
            'equable',
            'succinct',
            'conscious',
            'sorrowful',
            'worldly',
            'watery',
            'amused',
            'radiant',
            'pointed',
            'ready',
            'tidy',
            'rapid',
            'free',
            'parallel',
            'celebrated',
            'outstanding',
            'reminiscent',
            'capable',
            'abashed',
            'filthy',
            'dismal',
            'corny',
            'finicky',
            'dapper',
            'cultured',
            'taut',
            'smooth',
            'sleepy',
            'rainy',
            'faraway',
            'slippery',
            'unlawful',
            'sassy',
            'skeletal',
            'worthy',
            'silver',
            'memorable',
            'bluish',
            'shimmering',
            'moral',
            'subdued',
            'womanly',
            'accessible',
            'future',
            'unequal',
            'deeply',
            'important',
            'absurd',
            'gloomy',
            'whirlwind',
            'wealthy',
            'decisive',
            'general',
            'appropriate',
            'spotted',
            'cluttered',
            'mediocre',
            'shut',
            'humongous',
            'synonymous',
            'bored',
            'fuzzy',
            'superb',
            'spiffy',
            'imaginary',
            'complex',
            'shocked',
            'spherical',
            'steadfast',
            'stereotyped',
            'digital',
            'puzzled',
            'definite',
            'cruel',
            'fierce',
            'purple',
            'foamy',
            'elliptical',
            'symptomatic',
            'circular',
            'superficial',
            'sunny',
            'torn',
            'wakeful',
            'sore',
            'combative',
            'sweet',
            'submissive',
            'testy',
            'splendid',
            'phobic',
            'proper',
            'edible',
            'swift',
            'hopeful',
            'sugary',
            'stupendous',
            'fussy',
            'unsuitable',
            'absorbed',
            'tangible',
            'eminent',
            'smug',
            'idiotic',
            'awful',
            'attentive',
            'ceaseless',
            'acrobatic',
            'skinny',
            'corrupt',
            'dazzling',
            'sable',
            'every',
            'misty',
            'envious',
            'accidental',
            'French',
            'harmful',
            'blaring',
            'arid',
            'dangerous',
            'shabby',
            'definitive',
            'unwitting',
            'stingy',
            'impressive',
            'amusing',
            'hurtful',
            'severe',
            'studious',
            'secretive',
            'soupy',
            'grown',
            'solid',
            'frozen',
            'awkward',
            'anchored',
            'impolite',
            'spry',
            'authorized',
            'bright',
            'double',
            'acceptable',
            'sniveling',
            'tenuous',
            'courteous',
            'debonair',
            'adjoining',
            'tremendous',
            'shameful',
            'dreary',
            'ajar',
            'strict',
            'plant',
            'scarce',
            'snarling',
            'teeming',
            'recent',
            'scared',
            'spiky',
            'superior',
            'antique',
            'favorable',
            'slow',
            'pleasing',
            'foregoing',
            'specific',
            'scornful',
            'suspicious',
            'second',
            'adored',
            'squalid',
            'supportive',
            'avaricious',
            'secondary',
            'royal',
            'athletic',
            'shaky',
            'some'
        ]
        const animals = [
            'cowbird',
            'kite',
            'hoopoe',
            'vulture',
            'whapuku',
            'yearling',
            'narwhal',
            'nandu',
            'cassowary',
            'cooter',
            'iguana',
            'spoonbill',
            'waterstrider',
            'davidstiger',
            'erne',
            'shark',
            'pachyderm',
            'diplodocus',
            'jerboa',
            'invisiblerail',
            'dipper',
            'tick',
            'junco',
            'frenchbulldog',
            'nightingale',
            'moa',
            'cutworm',
            'goosefish',
            'bullfrog',
            'urutu',
            'jay',
            'nightjar',
            'zander',
            'redhead',
            'rockrat',
            'bittern',
            'coqui',
            'tilefish',
            'xantus',
            'woodborer',
            'xeme',
            'nyala',
            'guanaco',
            'foxhound',
            'jabiru',
            'aurochs',
            'kangaroo',
            'ichthyosaurs',
            'janenschia',
            'bellfrog',
            'queenconch',
            'bushviper',
            'pitbull',
            'ynambu',
            'xenoposeidon',
            'skua',
            'papillon',
            'uakari',
            'urchin',
            'wingedmagpie',
            'canary',
            'warbler',
            'inganue',
            'gelada',
            'yapok',
            'nene',
            'unau',
            'yellowlegs',
            'zebu',
            'nase',
            'titi',
            'achillestang',
            'equine',
            'woodpecker',
            'cub',
            'vasesponge',
            'stingray',
            'devilfish',
            'hamadryas',
            'aoudad',
            'esok',
            'titmouse',
            'ostrich',
            'dwarfrabbit',
            'roan',
            'javalina',
            'brontosaurus',
            'creamdraft',
            'quoll',
            'pilchard',
            'rooster',
            'vireo',
            'lamprey',
            'kawala',
            'viper',
            'dikkops',
            'zebra',
            'blacklab',
            'urson',
            'quahog',
            'finwhale',
            'xanthareel',
            'komondor',
            'parrotlet',
            'upupa',
            'quarterhorse',
            'turkey',
            'olingo',
            'ovenbird',
            'lcont',
            'vaquita',
            'urial',
            'quetzal',
            'nilgai',
            'louse',
            'joey',
            'graywolf',
            'bushsqueaker',
            'kitten',
            'quadrisectus',
            'tiger',
            'schnauzer',
            'cockatiel',
            'dorking',
            'xrayfish',
            'cony',
            'robin',
            'nightheron',
            'yucker',
            'vixen',
            'coot',
            'bustard',
            'coelacanth',
            'watussi',
            'hornedviper',
            'vampirebat',
            'bluejay',
            'duiker',
            'naga',
            'hamster',
            'boar',
            'wolfhound',
            'mantaray',
            'kitty',
            'goat',
            'nubiangoat',
            'turtledove',
            'elkhound',
            'acaciarat',
            'dog',
            'drake',
            'goldfish',
            'eelelephant',
            'gelding',
            'llama',
            'umbrette',
            'trout',
            'otter',
            'xenopus',
            'yeti',
            'trumpeterbird',
            'yellowthroat',
            'goblinshark',
            'volvox',
            'maiasaura',
            'quetzalcoatlus',
            'goldfinch',
            'whiterhino',
            'goral',
            'noddy',
            'lion',
            'quokka',
            'tarsier',
            'wyvern',
            'ichneumonfly',
            'chickadee',
            'ichthyostega',
            'gardensnake',
            'vole',
            'xenurine',
            'ostracod',
            'lynx',
            'lobster',
            'hog',
            'honeybadger',
            'backswimmer',
            'reptile',
            'ram',
            'racerunner',
            'shrike',
            'noctule',
            'molesnake',
            'flatfish',
            'okapi',
            'whale',
            'blowfish',
            'whoopingcrane',
            'noctilio',
            'hyena',
            'borzoi',
            'ibizanhound',
            'dugong',
            'myna',
            'rattail',
            'verdin',
            'dodo',
            'guillemot',
            'axisdeer',
            'harborporpoise',
            'sablefish',
            'yak',
            'nurseshark',
            'krill',
            'cur',
            'flea',
            'hoatzin',
            'mite',
            'mallard',
            'whelp',
            'koalabear',
            'teal',
            'thrush',
            'orangutan',
            'wireworm',
            'yaffle',
            'jackal',
            'glowworm',
            'eland',
            'flyingfox',
            'anura',
            'chafer',
            'xoni',
            'fox',
            'howlermonkey',
            'merlin',
            'xiaosaurus',
            'cottonmouth',
            'veery',
            'baboon',
            'huemul',
            'olm',
            'noolbenger',
            'amoeba',
            'tanager',
            'opossum',
            'oryx',
            'leopardseal',
            'butterfly',
            'nightcrawler',
            'kinkajou',
            'xiphiasgladius',
            'koala',
            'cuckoo',
            'teledu',
            'smelts',
            'wirehair',
            'nettlefish',
            'rabidsquirrel',
            'dorado',
            'pewee',
            'falcon',
            'zorilla',
            'pterodactyl',
            'islandwhistler',
            'chevrotain',
            'chital',
            'polyturator',
            'vicuna',
            'trumpeterswan',
            'jaguarundi',
            'nymph',
            'horseshoebat',
            'pullet',
            'draughthorse',
            'yellowhammer',
            'electriceel',
            'rook',
            'hen',
            'towhee',
            'dotterel',
            'farmdog',
            'grouper',
            'kakapo',
            'lunamoth',
            'walkingstick',
            'axolotl',
            'muskrat',
            'puffin',
            'inepython',
            'peacock',
            'chihuahua',
            'jellyfish',
            'leech',
            'zeren',
            'springtail',
            'sheep',
            'leafbird',
            'termite',
            'kronosaurus',
            'halcyon',
            'molly',
            'lemur',
            'kleekai',
            'annelid',
            'toadfish',
            'whooper',
            'xenarthra',
            'banteng',
            'egret',
            'ridgeback',
            'puma',
            'hornedfrog',
            'quelea',
            'hippopotamus',
            'pangolin',
            'piranha',
            'ruddyduck',
            'vipersquid',
            'ungulate',
            'jackrabbit',
            'waterboatman',
            'kusimanse',
            'drongo',
            'ewe',
            'auklet',
            'loon',
            'eft',
            'massasauga',
            'frigatebird',
            'jaguar',
            'urus',
            'quagga',
            'xenops',
            'fairyfly',
            'andeancondor',
            'venomoussnake',
            'lice',
            'flycatcher',
            'lorikeet',
            'indigobunting',
            'eel',
            'hammerkop',
            'puppy',
            'elephant',
            'gemsbuck',
            'eagle',
            'dassie',
            'appaloosa',
            'firecrest',
            'yardant',
            'harpyeagle',
            'auk',
            'giantpetrel',
            'kodiakbear',
            'panther',
            'wobbegongshark',
            'manta',
            'jingle',
            'raccoon',
            'ling',
            'gilamonster',
            'pronghorn',
            'stag',
            'spreadwing',
            'sugarglider',
            'whippoorwill',
            'jumpingbean',
            'mamba',
            'kookaburra',
            'flounder',
            'fawn',
            'dungenesscrab',
            'rasbora',
            'redstart',
            'nighthawk',
            'foxterrier',
            'donkey',
            'guernseycow',
            'waterdogs',
            'trogon',
            'pintail',
            'nerka',
            'kiskadee',
            'dingo',
            'jacana',
            'vase',
            'lemonshark',
            'bear',
            'imperatorangel',
            'emeraldlizard',
            'ibisbill',
            'earthworm',
            'wolf',
            'abat',
            'clam',
            'reindeer',
            'bobolink',
            'dachshund',
            'ray',
            'cuttlefish',
            'gannet',
            'waterbug',
            'glasslizard',
            'dore',
            'orca',
            'zenaida',
            'dwarfmongoose',
            'squeaker',
            'allosaurus',
            'spaniel',
            'antbear',
            'kelpie',
            'hawk',
            'megaraptor',
            'imago',
            'fowl',
            'wisent',
            'rhea',
            'bovine',
            'vulpesvulpes',
            'bream',
            'emperorpenguin',
            'skimmer',
            'polyp',
            'hypacrosaurus',
            'kouprey',
            'meadowlark',
            'oxen',
            'grunion',
            'urubu',
            'tortoise',
            'emperorshrimp',
            'fluke',
            'iriomotecat',
            'nematode',
            'beaver',
            'carp',
            'ragfish',
            'husky',
            'schapendoes',
            'sifaka',
            'armadillo',
            'erin',
            'nuthatch',
            'bunting',
            'lizard',
            'feline',
            'kakarikis',
            'tapaculo',
            'wattlebird',
            'groundhornbill',
            'mosasaur',
            'walleye',
            'spiketail',
            'thoroughbred',
            'zebrafish',
            'terrapin',
            'viperfish',
            'seagull',
            'motmot',
            'guineafowl',
            'herald',
            'degu',
            'plankton',
            'lacewing',
            'bufflehead',
            'porpoise',
            'kittiwake',
            'pooch',
            'barasingha',
            'greatdane',
            'starfish',
            'izuthrush',
            'osprey',
            'billygoat',
            'alpaca',
            'koi',
            'discus',
            'xiphosuran',
            'limpet',
            'kinglet',
            'fantail',
            'boilweevil',
            'unicorn',
            'imperialeagle',
            'gorilla',
            'hedgehog',
            'thrip',
            'galah',
            'whiteeye',
            'swallow',
            'ocelot',
            'grizzlybear',
            'sawfish',
            'bird',
            'horseshoecrab',
            'earwig',
            'deinonychus',
            'deer',
            'rainbowtrout',
            'pinscher',
            'monkfish',
            'tsetsefly',
            'plainsqueaker',
            'woodchuck',
            'harborseal',
            'equestrian',
            'eyra',
            'giraffe',
            'tegus',
            'ant',
            'iggypops',
            'xanclomys',
            'aruanas',
            'zebradove',
            'rail',
            'cock',
            'manatee',
            'koodoo',
            'bluet',
            'acouchi',
            'catbird',
            'ratfish',
            'fossa',
            'oxpecker',
            'kob',
            'newt',
            'honeycreeper',
            'longspur',
            'quail',
            'roebuck',
            'oropendola',
            'pomeranian',
            'lemming',
            'goose',
            'quillback',
            'camel',
            'hagfish',
            'leveret',
            'hind',
            'aardvark',
            'majungatholus',
            'heifer',
            'chuckwalla',
            'gopher',
            'meadowhawk',
            'antelope',
            'sparrow',
            'gallinule',
            'chiffchaff',
            'hyrax',
            'caribou',
            'nandoo',
            'johndory',
            'gourami',
            'piedstarling',
            'langur',
            'ferret',
            'gull',
            'wallaroo',
            'giantschnauzer',
            'devil',
            'oyster',
            'hare',
            'hummingbird',
            'guineapig',
            'sable',
            'nauplius',
            'trumpetfish',
            'smew',
            'basilisk',
            'solenodon',
            'painthorse',
            'triceratops',
            'siskin',
            'wallaby',
            'pigeon',
            'pupfish',
            'xerus',
            'rottweiler',
            'redpoll',
            'malamute',
            'barebirdbat',
            'rhino',
            'wilddog',
            'zopilote',
            'atlasmoth',
            'ivorygull',
            'hornet',
            'tragopan',
            'kid',
            'waterspaniel',
            'hookersealion',
            'sakimonkey',
            'haddock',
            'killdeer',
            'paca',
            'rabbit',
            'bluewhale',
            'gangesdolphin',
            'gar',
            'hart',
            'pointer',
            'cormorant',
            'tench',
            'salmon',
            'raven',
            'fallowdeer',
            'wren',
            'gosling',
            'waterthrush',
            'eeve',
            'velvetworm',
            'karakul',
            'weasel',
            'fugu',
            'gnu',
            'ibadanmalimbe',
            'peafowl',
            'nutria',
            'squamata',
            'watermoccasin',
            'mangabey',
            'basil',
            'finch',
            'microvenator',
            'cockerspaniel',
            'human',
            'tahr',
            'dassierat',
            'shihtzu',
            'xiphias',
            'thrasher',
            'xraytetra',
            'airedale',
            'lowchen',
            'racer',
            'ptarmigan',
            'partridge',
            'mammoth',
            'toucan',
            'hydra',
            'hoki',
            'blackbird',
            'vulpesvelox',
            'tuna',
            'zebrafinch',
            'tamarin',
            'tomtit',
            'icefish',
            'killifish',
            'budgie',
            'wrenchbird',
            'wigeon',
            'garpike',
            'anaconda',
            'massospondylus',
            'hanumanmonkey',
            'zooplankton',
            'killerwhale',
            'palmsquirrel',
            'gartersnake',
            'fish',
            'gyrfalcon',
            'warmblood',
            'turtle',
            'copperhead',
            'morayeel',
            'affenpinscher',
            'bluegill',
            'monkey',
            'vervet',
            'swift',
            'anchovy',
            'groundsquirrel',
            'frog',
            'cygnet',
            'neontetra',
            'anhinga',
            'gordonsetter',
            'firefly',
            'velvetcrab',
            'mullet',
            'homalocephale',
            'tapir',
            'lionfish',
            'hapuku',
            'lungfish',
            'pinemarten',
            'cavy',
            'herring',
            'rat',
            'greyhound',
            'leafwing',
            'owlbutterfly',
            'albino',
            'bellsnake',
            'barbet',
            'daygecko',
            'sunbittern',
            'grayfox',
            'amphiuma',
            'hammerheadbird',
            'oriole',
            'amphibian',
            'umbrellabird',
            'cero',
            'stickleback',
            'conch',
            'cheetah',
            'fisheagle',
            'dowitcher',
            'crane',
            'groundjay',
            'parakeet',
            'ibex',
            'mussel',
            'dove',
            'phalarope',
            'waxwing',
            'glassfrog',
            'hogdeer',
            'gadwall',
            'pika',
            'elk',
            'ox',
            'ilsamochadegu',
            'lark',
            'wombat',
            'hadrosaurus',
            'dunnart',
            'heron',
            'mantis',
            'dragon',
            'fruitbat',
            'bug',
            'grub',
            'barbel',
            'crocodile',
            'honeyeater',
            'woodstorks',
            'ichidna',
            'shorebird',
            'blackfish',
            'wildebeast',
            'mollusk',
            'merganser',
            'cottontail',
            'velociraptor',
            'treeboa',
            'fiddlercrab',
            'gaur',
            'dolphin',
            'mole',
            'jenny',
            'nudibranch',
            'mussaurus',
            'weaverbird',
            'sow',
            'godwit',
            'queensnake',
            'flicker',
            'squid',
            'samoyeddog',
            'avians',
            'hackee',
            'elver',
            'skylark',
            'doctorfish',
            'springpeeper',
            'muskox',
            'flies',
            'nag',
            'grison',
            'acornweevil',
            'duckling',
            'pharaohhound',
            'monoclonius',
            'scarab',
            'fieldspaniel',
            'seabird',
            'snail',
            'duckbillcat',
            'kagu',
            'civet',
            'ermine',
            'nutcracker',
            'cirriped',
            'pinniped',
            'nagapies',
            'stallion',
            'coyote',
            'bullshark',
            'walrus',
            'kingsnake',
            'owl',
            'bergerpicard',
            'shoveler',
            'bluefintuna',
            'shorthair',
            'avocet',
            'komododragon',
            'gerbil',
            'gonolek',
            'greatargus',
            'wildass',
            'pheasant',
            'cougar',
            'wildcat',
            'mongrel',
            'starling',
            'polliwog',
            'cow',
            'needletail',
            'arkshell',
            'shrimp',
            'purplemarten',
            'aegeancat',
            'curl',
            'dairycow',
            'grayling',
            'cranefly',
            'prairiedog',
            'emu',
            'dunlin',
            'macaque',
            'doe',
            'comet',
            'gypsymoth',
            'leafcutterant',
            'nandine',
            'mice',
            'hornshark',
            'crustacean',
            'newtnutria',
            'coney',
            'fishingcat',
            'gemsbok',
            'crayfish',
            'magpie',
            'warthog',
            'rodent',
            'scorpion',
            'mockingbird',
            'incatern',
            'pipit',
            'tayra',
            'gecko',
            'ankole',
            'serval',
            'bluebird',
            'terrier',
            'urva',
            'parrot',
            'possum',
            'shearwater',
            'hatchetfish',
            'mudpuppy',
            'phoenix',
            'crocodileskink',
            'tigershark',
            'addax',
            'grayreefshark',
            'bluefish',
            'conure',
            'huia',
            'laika',
            'goshawk',
            'lobo',
            'kudu',
            'indri',
            'moorhen',
            'flyingfish',
            'graysquirrel',
            'flyingsquirrel',
            'bufeo',
            'deermouse',
            'tarpan',
            'pondskater',
            'seaurchin',
            'kingfisher',
            'egg',
            'anole',
            'stonefly',
            'cobra',
            'ratsnake',
            'anteater',
            'coursinghounds',
            'cats',
            'fairybluebird',
            'sturgeon',
            'marmot',
            'adder',
            'barb',
            'thunderbird',
            'loris',
            'macaw',
            'agouti',
            'lhasaapso',
            'ape',
            'wreckfish',
            'andeancat',
            'goitered',
            'thylacine',
            'galago',
            'hartebeest',
            'collie',
            'shepherd',
            'dikdik',
            'hellbender',
            'mara',
            'senegalpython',
            'octopus',
            'leonberger',
            'meerkat',
            'rainbowfish',
            'hoiho',
            'sora',
            'steed',
            'sealion',
            'hake',
            'copepod',
            'waterbuck',
            'pikeperch',
            'grosbeak',
            'larva',
            'burro',
            'buzzard',
            'leafhopper',
            'brant',
            'kentrosaurus',
            'clumber',
            'tuatara',
            'mayfly',
            'cockatoo',
            'mandrill',
            'cardinal',
            'mutt',
            'canine',
            'armyant',
            'screamer',
            'flamingo',
            'islandcanary',
            'pekingese',
            'rhinoceros',
            'turaco',
            'tern',
            'dinosaur',
            'spadefish',
            'polecat',
            'skunk',
            'boto',
            'germanshepherd',
            'capybara',
            'poodle',
            'kingbird',
            'finnishspitz',
            'sphinx',
            'mammal',
            'takin',
            'nabarlek',
            'silverfox',
            'eyas',
            'raptors',
            'marten',
            'guppy',
            'freshwaterclam',
            'kiwi',
            'mantid',
            'siamang',
            'frilledlizard',
            'duck',
            'numbat',
            'python',
            'hectorsdolphin',
            'snowmonkey',
            'spotteddolphin',
            'hapuka',
            'needlefish',
            'nakedmolerat',
            'groundhog',
            'mealworm',
            'squirrel',
            'chanticleer',
            'lamb',
            'moose',
            'primate',
            'minnow',
            'firesalamander',
            'bettong',
            'chicken',
            'reynard',
            'hornedtoad',
            'scarletibis',
            'milksnake',
            'blackbuck',
            'crossbill',
            'hamadryad',
            'cornsnake',
            'illadopsis',
            'rockpython',
            'eskimodog',
            'globefish',
            'adouri',
            'macropod',
            'agama',
            'harrierhawk',
            'roller',
            'caudata',
            'mynah',
            'hairstreak',
            'coypu',
            'liger',
            'penguin',
            'mouflon',
            'arawana',
            'kitfox',
            'dalmatian',
            'morpho',
            'carpenterant',
            'slothbear',
            'caiman',
            'toad',
            'setter',
            'argusfish',
            'honeybee',
            'mollies',
            'grouse',
            'armedcrab',
            'perch',
            'eider',
            'drever',
            'alligator',
            'sunbear',
            'booby',
            'blobfish',
            'fanworms',
            'pike',
            'spottedowl',
            'shelduck',
            'turnstone',
            'hylaeosaurus',
            'treecreeper',
            'swellfish',
            'clawedfrog',
            'panda',
            'pupa',
            'cowrie',
            'wrasse',
            'fireant',
            'cricket',
            'humpbackwhale',
            'chick',
            'sanddollar',
            'widgeon',
            'chameleon',
            'marabou',
            'squab',
            'moray',
            'marlin',
            'acornbarnacle',
            'freshwatereel',
            'mantisray',
            'wryneck',
            'bonobo',
            'whitepelican',
            'woodcock',
            'prawn',
            'megalosaurus',
            'makoshark',
            'ladybird',
            'topi',
            'whimbrel',
            'marmoset',
            'angelfish',
            'echidna',
            'hyracotherium',
            'muntjac',
            'anemone',
            'blackgoby',
            'goldeneye',
            'boutu',
            'myotis',
            'stiger',
            'willet',
            'cob',
            'beagle',
            'mastiff',
            'kestrel',
            'greathornedowl',
            'dromedary',
            'pussycat',
            'sidewinder',
            'binturong',
            'gentoopenguin',
            'sandpiper',
            'astarte',
            'bongo',
            'phoebe',
            'bighorn',
            'monkseal',
            'manxcat',
            'pipistrelle',
            'mustang',
            'damselfly',
            'horse',
            'dormouse',
            'harrier',
            'harvestmen',
            'caecilian',
            'gerenuk',
            'bullmastiff',
            'abalone',
            'sponge',
            'barnacle',
            'buck',
            'shrew',
            'pelican',
            'tattler',
            'robberfly',
            'flyinglemur',
            'elephantseal',
            'bass',
            'goa',
            'smoushond',
            'bronco',
            'pufferfish',
            'sunbird',
            'apatosaur',
            'albatross',
            'genet',
            'sheepdog',
            'bichonfrise',
            'buffalo',
            'bubblefish',
            'furseal',
            'pug',
            'platypus',
            'fulmar',
            'porcupine',
            'mouse',
            'curassow',
            'cat',
            'curlew',
            'grackle',
            'limpkin',
            'waterbuffalo',
            'amurminnow',
            'impala',
            'piglet',
            'sardine',
            'angwantibo',
            'caterpillar',
            'isopod',
            'foal',
            'rattlesnake',
            'tinamou',
            'boubou',
            'polarbear',
            'aztecant',
            'iguanodon',
            'harvestmouse',
            'silverfish',
            'minibeast',
            'hogget',
            'chinchilla',
            'plover',
            'armyworm',
            'amberpenshell',
            'ibis',
            'weevil',
            'slug',
            'cattle',
            'baiji',
            'alleycat',
            'tenrec',
            'antlion',
            'cattledog',
            'lovebird',
            'akitainu',
            'condor',
            'fieldmouse',
            'crab',
            'halicore',
            'hypsilophodon',
            'seal',
            'sheldrake',
            'blesbok',
            'stilt',
            'gemclam',
            'arthropod',
            'tadpole',
            'monarch',
            'glassfish',
            'halibut',
            'barracuda',
            'paddlefish',
            'mule',
            'gharial',
            'fennecfox',
            'taruca',
            'seriema',
            'caracal',
            'gander',
            'ammonite',
            'pygmy',
            'desertpupfish',
            'chamois',
            'ayeaye',
            'fly',
            'serpent',
            'crow',
            'ghostshrimp',
            'springbok',
            'archerfish',
            'solitaire',
            'horsemouse',
            'roach',
            'gallowaycow',
            'gavial',
            'pony',
            'midge',
            'piedkingfisher',
            'scaup',
            'chimneyswift',
            'seaslug',
            'whippet',
            'gazelle',
            'scallop',
            'stork',
            'seahog',
            'pterosaurs',
            'beauceron',
            'pig',
            'clingfish',
            'puffer',
            'seahorse',
            'harpseal',
            'wolverine',
            'argali',
            'planthopper',
            'worm',
            'bluebottle',
            'bat',
            'leopard',
            'coral',
            'chimpanzee',
            'cicada',
            'salamander',
            'dogfish',
            'ass',
            'bats',
            'angora',
            'cleanerwrasse',
            'sloth',
            'calf',
            'bull',
            'frogmouth',
            'skipper',
            'coati',
            'hound',
            'murrelet',
            'mink',
            'aardwolf',
            'saiga',
            'bison',
            'cusimanse',
            'alpinegoat',
            'basenji',
            'trex',
            'aracari',
            'silkworm',
            'bobcat',
            'bilby',
            'mongoose',
            'songbird',
            'darwinsfox',
            'dromaeosaur',
            'snipe',
            'blackfly',
            'peccary',
            'colt',
            'stud',
            'bluetang',
            'aidi',
            'nautilus',
            'roadrunner',
            'stegosaurus',
            'saddlebred',
            'bobwhite',
            'sanderling',
            'bushbaby',
            'mapturtle',
            'sambar',
            'mare',
            'waterdragons',
            'canvasback',
            'chipmunk',
            'snake',
            'badger',
            'blueshark',
            'bactrian',
            'betafish',
            'pittabird',
            'swallowtail',
            'stoat',
            'boa',
            'hermitcrab',
            'hornbill',
            'bumblebee',
            'chupacabra',
            'drafthorse',
            'steer',
            'bulldog',
            'gibbon',
            'natterjacktoad',
            'silkyterrier',
            'sapsucker',
            'bobtail',
            'bandicoot',
            'babirusa',
            'mastodon',
            'spitz',
            'creature',
            'cod',
            'catfish',
            'arrowana',
            'borer',
            'grebe',
            'goldencat',
            'bee',
            'swordfish',
            'mousebird',
            'sunfish',
            'swan',
            'seamonkey',
            'schipperke',
            'skink',
            'cuscus',
            'scoter',
            'bunny'
        ]

        const random = new SeededRandom(seed)

        let adjective = adjectives[random.nextInt(adjectives.length)]
        let animal = animals[random.nextInt(animals.length)]
        const number = random.nextInt(1000)

        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)

        return `${adjective}${animal}${number}`
    }
}

function tinyKeyStringToHuman(string: string): string {
    return string
        .split('+')
        .map((key) => {
            if (key === '$mod') return 'Ctrl'
            return key
        })
        .join(' + ')
}

const ResultItem = forwardRef(
    (
        {
            action,
            active,
            currentRootActionId
        }: {
            action: ActionImpl
            active: boolean
            currentRootActionId: ActionId
        },
        ref: Ref<HTMLDivElement>
    ): JSX.Element => {
        const ancestors = useMemo(() => {
            if (!currentRootActionId) return action.ancestors
            const index = action.ancestors.findIndex(
                (ancestor) => ancestor.id === currentRootActionId
            )
            // +1 removes the currentRootAction; e.g.
            // if we are on the "Set theme" parent action,
            // the UI should not display "Set theme… > Dark"
            // but rather just "Dark"
            return action.ancestors.slice(index + 1)
        }, [action.ancestors, currentRootActionId])

        return (
            <div
                ref={ref}
                className={`flex justify-between px-4 rounded-md  ${active ? 'bg-primary text-dark' : 'bg-dark bg-opacity-50 text-light'}`}
            >
                <div className="description">
                    {action.icon && action.icon}
                    <div>
                        <div>
                            {ancestors.length > 0 &&
                                ancestors.map((ancestor) => (
                                    <Fragment key={ancestor.id}>
                                        <span>{ancestor.name}</span>
                                        <span>&rsaquo;</span>
                                    </Fragment>
                                ))}
                            <span>{action.name}</span>
                        </div>
                        {action.subtitle && <span>{action.subtitle}</span>}
                    </div>
                </div>
                {action.shortcut?.length ? (
                    <div className="shortcut">
                        {action.shortcut.map((sc) => (
                            <kbd key={sc}>{tinyKeyStringToHuman(sc)}</kbd>
                        ))}
                    </div>
                ) : null}
            </div>
        )
    }
)

ResultItem.displayName = 'ResultItem'

interface RenderResultsProps {
    className?: string
}

function RenderResults({ className }: RenderResultsProps): JSX.Element {
    const { results, rootActionId } = useMatches()

    return (
        <KBarResults
            items={results}
            onRender={({ item, active }) =>
                typeof item === 'string' ? (
                    <div className={active ? `${className} active` : className}>{item}</div>
                ) : (
                    <ResultItem action={item} active={active} currentRootActionId={rootActionId} />
                )
            }
        />
    )
}

function CommandBar(): JSX.Element {
    return (
        <KBarPortal>
            <KBarPositioner className="backdrop-blur-sm">
                <KBarAnimator className="w-2/3">
                    <KBarSearch className="w-full bg-light border-none p-4 rounded-2xl placeholder:text-dark focus:bg-primary focus:outline-none focus:placeholder:text-light selection:bg-secondary" />
                    <RenderResults className=" bg-light bg-opacity-50 rounded-md px-2 py-1 box-content" />
                </KBarAnimator>
            </KBarPositioner>
        </KBarPortal>
    )
}

enum IconKind {
    Text,
    Svg,
    Image
}

function getIconData(dataUrl): [string, IconKind] {
    const svgStart = 'data:image/svg+xml;base64,'
    const pngStart = 'data:image/png;base64,'
    const jpegStart = 'data:image/jpeg;base64,'
    let kind
    let data
    if (dataUrl.startsWith(svgStart)) {
        kind = IconKind.Svg
        data = atob(dataUrl.substring(svgStart.length))
    } else if (dataUrl.startsWith(pngStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(pngStart.length));
        data = dataUrl
    } else if (dataUrl.startsWith(jpegStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(jpegStart.length));
        data = dataUrl
    } else {
        kind = IconKind.Text
        data = dataUrl
    }
    return [data, kind]
}

const GraphConfig = {
    NodeTypes: {
        piece: {
            typeText: '',
            shapeId: '#piece',
            shape: (
                <symbol
                    className="piece"
                    viewBox="0 0 50 50"
                    height="40"
                    width="40"
                    id="piece"
                    key="0"
                >
                    <circle cx="25" cy="25" r="24"></circle>
                </symbol>
            )
        }
    },
    NodeSubtypes: {},
    EdgeTypes: {
        attraction: {
            shapeId: '#attraction',
            shape: (
                <symbol viewBox="0 0 50 50" id="attraction" key="0">
                    {/* <circle cx="25" cy="25" r="8" fill="currentColor"> </circle> */}
                </symbol>
            )
        }
    }
}

interface IPieceNode extends INode {
    piece: PieceInput
}

interface IAttractionEdge extends IEdge {
    attraction: AttractionInput
}

interface IDraft extends IGraphInput {
    name?: string
    explanation?: string
    nodes: IPieceNode[]
    edges: IAttractionEdge[]
}

const NODE_KEY = 'id' // Allows D3 to correctly update DOM

const sample: IDraft = {
    nodes: [
        {
            id: 'b',
            title: '',
            icon: '🏫',
            x: 0,
            y: 0,
            type: 'piece',
            piece: {
                id: 'b',
                type: {
                    name: 'base'
                }
            }
        },
        {
            id: 's',
            title: '',
            icon: '🛗',
            x: 20,
            y: 70,
            type: 'piece',
            piece: {
                id: 's',
                type: {
                    name: 'shaft',
                    qualities: [
                        {
                            name: 'storeys',
                            value: '11',
                            unit: null
                        }
                    ]
                }
            }
        },
        {
            id: 'cp',
            title: '',
            icon: '🎓',
            x: 150,
            y: 50,
            type: 'piece',
            piece: {
                id: 'cp',
                type: {
                    name: 'capital'
                }
            }
        },
        {
            id: 'c1',
            title: '',
            icon: 'data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgdmVyc2lvbj0iMS4xIgogICB3aWR0aD0iMzIxLjk0NzQycHQiCiAgIGhlaWdodD0iMzE0Ljk4MzU4cHQiCiAgIHZpZXdCb3g9IjAgMCAzMjEuOTQ3NDEgMzE0Ljk4MzU4IgogICBvdmVyZmxvdz0idmlzaWJsZSIKICAgaWQ9InN2Zzg4IgogICBzb2RpcG9kaTpkb2NuYW1lPSJjYXBzdWxlXzEuc3ZnIgogICBpbmtzY2FwZTp2ZXJzaW9uPSIxLjMuMiAoMDkxZTIwZSwgMjAyMy0xMS0yNSwgY3VzdG9tKSIKICAgeG1sbnM6aW5rc2NhcGU9Imh0dHA6Ly93d3cuaW5rc2NhcGUub3JnL25hbWVzcGFjZXMvaW5rc2NhcGUiCiAgIHhtbG5zOnNvZGlwb2RpPSJodHRwOi8vc29kaXBvZGkuc291cmNlZm9yZ2UubmV0L0RURC9zb2RpcG9kaS0wLmR0ZCIKICAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogICB4bWxuczpzdmc9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZGVmcwogICAgIGlkPSJkZWZzODgiIC8+CiAgPHNvZGlwb2RpOm5hbWVkdmlldwogICAgIGlkPSJuYW1lZHZpZXc4OCIKICAgICBwYWdlY29sb3I9IiNmZmZmZmYiCiAgICAgYm9yZGVyY29sb3I9IiMwMDAwMDAiCiAgICAgYm9yZGVyb3BhY2l0eT0iMC4yNSIKICAgICBpbmtzY2FwZTpzaG93cGFnZXNoYWRvdz0iMiIKICAgICBpbmtzY2FwZTpwYWdlb3BhY2l0eT0iMC4wIgogICAgIGlua3NjYXBlOnBhZ2VjaGVja2VyYm9hcmQ9IjAiCiAgICAgaW5rc2NhcGU6ZGVza2NvbG9yPSIjZDFkMWQxIgogICAgIGlua3NjYXBlOmRvY3VtZW50LXVuaXRzPSJwdCIKICAgICBpbmtzY2FwZTp6b29tPSIwLjc0MzE1NjU1IgogICAgIGlua3NjYXBlOmN4PSIyMjQuNzE3MTIiCiAgICAgaW5rc2NhcGU6Y3k9IjIzMC4wOTk1NyIKICAgICBpbmtzY2FwZTp3aW5kb3ctd2lkdGg9IjE5MjAiCiAgICAgaW5rc2NhcGU6d2luZG93LWhlaWdodD0iMTAxMCIKICAgICBpbmtzY2FwZTp3aW5kb3cteD0iLTYiCiAgICAgaW5rc2NhcGU6d2luZG93LXk9IjEwNzciCiAgICAgaW5rc2NhcGU6d2luZG93LW1heGltaXplZD0iMSIKICAgICBpbmtzY2FwZTpjdXJyZW50LWxheWVyPSJzdmc4OCIgLz4KICA8ZwogICAgIGlkPSJ3aW5kb3ciCiAgICAgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoLTEyOC45MzU4NiwtMjQ4LjQ0Njk5KSI+CiAgICA8cGF0aAogICAgICAgZD0ibSA0MjQuNzcxMzYsNDQ2Ljk5MzYgYyAwLjI5NDM0LDguMzk4NzMgLTMuMzcwMDgsMjAuODk0MDMgLTkuMzM2MzksMzEuNDE3MzUgLTUuOTY2MzEsMTAuNTIzMzIgLTE0LjIzNDUsMTkuMDc0NTggLTIyLjU0MDA3LDIzLjc5NDIyIC04LjMwNTU0LDQuNzE5NiAtMTYuNjQ4NDQsNS42MDc1MSAtMjIuNTQsMi4yMzI3NiAtNS44OTE1OCwtMy4zNzQ3NiAtOS4zMzE4LC0xMS4wMTIxOCAtOS4zMzY0LC0yMC42MzY2MyAtMC4wMDUsLTkuNjI0NDIgMy40MjY0LC0yMS4yMzU4IDkuMzM2NCwtMzEuNDE3MzYgNS45MSwtMTAuMTgxNTIgMTQuMjk5LC0xOC45MzMyIDIyLjU0LC0yMy43OTQyMiA4LjI0MTA2LC00Ljg2MDk5IDE2LjMzNDA3LC01LjgzMTM2IDIyLjQzODEsLTIuMjI2MTkgMy4wNTIwNCwxLjgwMjU3IDUuNjA2ODIsNC43NDkwMiA3LjIwMDIzLDguMzczNSAxLjU5MzQ3LDMuNjI0NDcgMi4yMjU1Nyw3LjkyNzA3IDIuMjM4MTMsMTIuMjU2NTcgeiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgxIiAvPgogIDwvZz4KICA8ZwogICAgIGlkPSJkb29yIgogICAgIHRyYW5zZm9ybT0idHJhbnNsYXRlKC0xMjguOTM1ODYsLTI0OC40NDY5OSkiPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTk0LjI4MDI5LDI5OS43NjI3IHYgOC40MzgzIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDIiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzEzLjg3MDI3IHYgNS42NjkzMSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDMiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzI1LjIwODg2IHYgNS42NjkyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDQiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzM2LjU0NzQ1IHYgNS42NjkyOSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDUiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzQ3Ljg4NjA1IHYgNS42NjkyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDYiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzU5LjIyNDYgdiA1LjY2OTMiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg3IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTk0LjI4MDI5LDM3MC41NjMyIHYgNS42NjkyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDgiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksMzgxLjkwMTggdiA1LjY2OTI4IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoOSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE5NC4yODAyOSwzOTMuMjQwMzYgdiA1LjY2OTI4IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMTAiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxOTQuMjgwMjksNDA0LjU3ODk1IHYgOC40MzgyOSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgxMSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE5NC4yODAyOSw0MTMuMDE3MjQgLTcuMzQxODcsNC4yMzg4MyIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgxMiIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE4Mi4wMjg2Niw0MjAuMDkwNyAtNC45MDk3NiwyLjgzNDY1IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMTMiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNzIuMjA5MTUsNDI1Ljc2IC00LjkwOTc0LDIuODM0NjQiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgxNCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE2Mi4zODk2Niw0MzEuNDI5MyAtNy4zNDE4OCw0LjIzODg1IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDE1IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTU1LjA0Nzc4LDMyMi40MTM2IDcuMzQxODgsLTQuMjM4ODMiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMTYiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNjcuMjk5NDEsMzE1LjM0MDE1IDQuOTA5NzQsLTIuODM0NjUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgxNyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE3Ny4xMTg5LDMwOS42NzA4NCA0LjkwOTc2LC0yLjgzNDY2IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMTgiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxODYuOTM4NDIsMzA0LjAwMTU2IDcuMzQxODcsLTQuMjM4ODYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMTkiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNTUuMDQ3NzgsNDM1LjY2ODE1IHYgLTguNDM4MjkiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMjAiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNTUuMDQ3NzgsNDIxLjU2MDU1IHYgLTUuNjY5MjgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgyMSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDE1NS4wNDc3OCw0MTAuMjIxOTUgViA0MDQuNTUyNyIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDIyIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTU1LjA0Nzc4LDM5OC44ODM0IHYgLTUuNjY5MyIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDIzIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTU1LjA0Nzc4LDM4Ny41NDQ4IHYgLTUuNjY5MjgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgyNCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE1NS4wNDc3OCwzNzYuMjA2MjQgdiAtNS42NjkyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDI1IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTU1LjA0Nzc4LDM2NC44Njc2NSB2IC01LjY2OTI5IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMjYiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNTUuMDQ3NzgsMzUzLjUyOTA1IHYgLTUuNjY5MzEiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgyNyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE1NS4wNDc3OCwzNDIuMTkwNSB2IC01LjY2OTMyIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMjgiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNTUuMDQ3NzgsMzMwLjg1MTkgdiAtOC40MzgzIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDI5IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTQ1LjIzOTY0LDQzMC4wMDU0IDkuODA4MTQsNS42NjI3NSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzMCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCwzMTYuNzUwODUgdiA4LjQzODMiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMzEiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNDUuMjM5NjQsMzMwLjg1ODQ2IHYgNS42NjkyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDMyIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTQ1LjIzOTY0LDM0Mi4xOTcwNSB2IDUuNjY5MjgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzMyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCwzNTMuNTM1NiB2IDUuNjY5MzMiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzNCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCwzNjQuODc0MiB2IDUuNjY5MzIiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzNSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCwzNzYuMjEyOCB2IDUuNjY5MjgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzNiIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCwzODcuNTUxMzYgdiA1LjY2OTMxIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9ImJ1dHQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoMzciIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNDUuMjM5NjQsMzk4Ljg4OTk1IHYgNS42NjkzMiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDM4IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTQ1LjIzOTY0LDQxMC4yMjg1NSB2IDUuNjY5MjgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGgzOSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE0NS4yMzk2NCw0MjEuNTY3MSB2IDguNDM4MyIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg0MCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE4NC40NzIxNywyOTQuMDk5OTggOS44MDgxMiw1LjY2MjcyIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIxLjQxNzMyIgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDQxIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTQ1LjIzOTY0LDMxNi43NTA4NSA5LjgwODE0LDUuNjYyNzUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNDIiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxODQuNDcyMTcsMjk0LjA5OTk4IC03LjM0MTg5LDQuMjM4ODIiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNDMiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNzIuMjIwNTIsMzAxLjE3MzQ2IC00LjkwOTc0LDIuODM0NjYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0iYnV0dCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg0NCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDE2Mi40MDEwMywzMDYuODQyNzQgLTQuOTA5NzYsMi44MzQ2NiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMS40MTczMiIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJidXR0IgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDQ1IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTUyLjU4MTUzLDMxMi41MTIwNSAtNy4zNDE4OSw0LjIzODgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjEuNDE3MzIiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNDYiIC8+CiAgPC9nPgogIDxnCiAgICAgaWQ9InNoZWxsIgogICAgIHRyYW5zZm9ybT0idHJhbnNsYXRlKC0xMjguOTM1ODYsLTI0OC40NDY5OSkiPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMzE2Ljg4MTkzLDQzOC40OTk1IGEgMjcuMDk5MTY5LDI3LjA5OTE2OSAwIDAgMCAyNC41MjAzMiwwIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDQ3IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMjQyLjI3MTQsMjUyLjAzNzMyIGEgMTcuMzM4NDksMTcuMzM4NDkgMCAwIDEgMTcuMDc0ODgsMC4xNDkxNCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg0OCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDEzOS4yODQ2MiwzMTEuNDkzNyAyNDIuMDA3OCwyNTIuMTg2NDUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNDkiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAzMTYuODgxOTMsNTUxLjc1NCBhIDI3LjA5OTE2OSwyNy4wOTkxNjkgMCAwIDAgMjQuNTIwMzIsMCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg1MCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDQ0OS40NjU5NCw0ODUuMjE2OTggYSAxNy4zMzg0ODgsMTcuMzM4NDg4IDAgMCAxIC04LjY2OTI0LDE1LjAxNTU5IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDUxIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Ik0gMzE2Ljg4MTkzLDU1MS43NTQgMTM1LjQzMTUyLDQ0Ni45OTM2IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDUyIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Ik0gMzI5LjE0MjEsNDE3LjI2NDI4IEEgMjcuMDk5MTY5LDI3LjA5OTE2OSAwIDAgMCAzMTYuODgxOTMsNDM4LjQ5OTUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNTMiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAyNTAuNjc3MDMsMjUzLjA0NTIxIGEgOC42Njk2MDQsOC42Njk2MDQgMCAwIDAgLTguNjY5MjMsLTAuODU4NzUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNTQiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAzNDEuNDAyMjUsNDM4LjQ5OTUgViA1NTEuNzU0IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDU1IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMzIwLjQ3Mjg0LDU1OS42OTEyIGEgOC43MzM0MTcsOC43MzM0MTcgMCAwIDEgLTMuNTkwOTEsLTcuOTM3MiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg1NiIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDM0MS40MDIyNSw0MzguNDk5NSA0NDQuMzg3NiwzNzkuMDQwODYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNTciIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxMzUuNDMxNTIsMzMzLjczOTA3IGEgOC43MzM0MTcsOC43MzM0MTcgMCAwIDEgLTUuMDc4MzQsLTcuMDc4NDMiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNTgiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAzNDEuNDAyMjUsNDM4LjQ5OTUgQSAyNy4wOTkxNjksMjcuMDk5MTY5IDAgMCAwIDMyOS4xNDIxLDQxNy4yNjQyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg1OSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDEzNS40MzE1MiwzMzMuNzM5MDcgViA0NDYuOTkzNiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg2MCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDEzNS40MzE1MiwzMzMuNzM5MDcgMzE2Ljg4MTkzLDQzOC40OTk1IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDYxIiAvPgogICAgPHBhdGgKICAgICAgIGQ9Ik0gMjU5LjM0NjI4LDI1Mi4xODY0NSA0NDAuNzk2NywzNTYuOTQ2OSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg2MiIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDQzMi4xMjc0NCwzNTcuODA1NjYgMjUwLjY3NzA1LDI1My4wNDUyMyIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg2MyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDQ0MC43OTY3LDM1Ni45NDY5IGEgMTcuMzM4NDg4LDE3LjMzODQ4OCAwIDAgMSA4LjY2OTI0LDE1LjAxNTU2IgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDY0IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTMwLjM1MzE4LDMyNi42NjA2NCBhIDE3LjMzODQ4OCwxNy4zMzg0ODggMCAwIDEgOC42NjkyNSwtMTUuMDE1NTYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNjUiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAzMjkuMTQyMDYsNDE3LjI2NDI4IDQzMi4xMjc0NCwzNTcuODA1NjYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNjYiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSA0NDAuNzk2Nyw1MDAuMjMyNTcgYSA4LjY2OTYwNCw4LjY2OTYwNCAwIDAgMCAzLjU5MDksLTcuOTM3MTciCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNjciIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSA0NDQuMzg3Niw0OTIuMjk1NDQgMzQxLjQwMjI1LDU1MS43NTQiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNjgiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSA0NDAuNzk2NywzNTYuOTQ2OSBhIDguNzMzNDE3LDguNzMzNDE3IDAgMCAwIC04LjY2OTI2LDAuODU4NzYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNjkiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSA0NDkuNDY1OSwzNzEuOTYyNDYgViA0ODUuMjE2OTgiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzAiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAzMzcuODExMyw1NTkuNjkxMiBhIDE3LjMzODQ4OCwxNy4zMzg0ODggMCAwIDEgLTE3LjMzODQ2LDAiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzEiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAyNTAuNjc3MDUsMjUzLjA0NTIzIDE0Ny42OTE2NywzMTIuNTAzODUiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzIiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAyNTkuMzQ2MjgsMjUyLjE4NjQ2IGEgOC43MzM0MTcsOC43MzM0MTcgMCAwIDAgLTguNjY5MjUsMC44NTg3NSIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg3MyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDQ0OS40NjU5NCwzNzEuOTYyNDYgYSA4LjczMzQxNyw4LjczMzQxNyAwIDAgMSAtNS4wNzgzNCw3LjA3ODQiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzQiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxMzAuMzUzMTgsNDM5LjkxNTIgYSA4LjY2OTYwNCw4LjY2OTYwNCAwIDAgMCA1LjA3ODM0LDcuMDc4NCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg3NSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDQ0MC43OTY3LDUwMC4yMzI1NCAzMzcuODExMyw1NTkuNjkxMTYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzYiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAzMjAuNDcyODQsNTU5LjY5MTE2IDEzOS4wMjI0Myw0NTQuOTMwNzYiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoNzciIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSA0MzIuMTI3NDQsMzU3LjgwNTY2IGEgMjcuMDk5MTY5LDI3LjA5OTE2OSAwIDAgMSAxMi4yNjAxNiwyMS4yMzUyIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDc4IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMTM1LjQzMTUyLDMzMy43MzkwNyBhIDI3LjA5OTE2OSwyNy4wOTkxNjkgMCAwIDEgMTIuMjYwMTYsLTIxLjIzNTIyIgogICAgICAgc3Ryb2tlPSIjMDAwMDAwIgogICAgICAgc3Ryb2tlLXdpZHRoPSIyLjgzNDY1IgogICAgICAgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIgogICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgIGZpbGw9Im5vbmUiCiAgICAgICBpZD0icGF0aDc5IiAvPgogICAgPHBhdGgKICAgICAgIGQ9Im0gMzQxLjQwMjI1LDU1MS43NTQgYSA4LjY2OTYwNCw4LjY2OTYwNCAwIDAgMSAtMy41OTA5NSw3LjkzNzIiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODAiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSA0NDQuMzg3Niw0OTIuMjk1NCBhIDguNzMzNDE3LDguNzMzNDE3IDAgMCAwIDUuMDc4MzQsLTcuMDc4NDIiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODEiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAxMzkuMDIyNDMsNDU0LjkzMDc2IEEgMTcuMzM4NDg4LDE3LjMzODQ4OCAwIDAgMSAxMzAuMzUzMTgsNDM5LjkxNTIiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODIiIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSAxMzAuMzUzMiw0MzkuOTE1MiBWIDMyNi42NjA2NCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg4MyIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDE0Ny42OTE2NywzMTIuNTAzODUgMzI5LjE0MjA2LDQxNy4yNjQyOCIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg4NCIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJtIDEzNS40MzE1Miw0NDYuOTkzNiBhIDguNzMzNDE3LDguNzMzNDE3IDAgMCAwIDMuNTkwOTEsNy45MzcxNiIKICAgICAgIHN0cm9rZT0iIzAwMDAwMCIKICAgICAgIHN0cm9rZS13aWR0aD0iMi44MzQ2NSIKICAgICAgIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIKICAgICAgIHN0cm9rZS1saW5lam9pbj0icm91bmQiCiAgICAgICBmaWxsPSJub25lIgogICAgICAgaWQ9InBhdGg4NSIgLz4KICAgIDxwYXRoCiAgICAgICBkPSJNIDMxNi44ODE5Myw0MzguNDk5NSBWIDU1MS43NTQiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODYiIC8+CiAgICA8cGF0aAogICAgICAgZD0ibSAxNDcuNjkxNjgsMzEyLjUwMzg1IGEgOC43MzM0MTcsOC43MzM0MTcgMCAwIDAgLTguNjY5MjUsLTAuODU4NzciCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODciIC8+CiAgICA8cGF0aAogICAgICAgZD0iTSA0NDQuMzg3NiwzNzkuMDQwODYgViA0OTIuMjk1NDQiCiAgICAgICBzdHJva2U9IiMwMDAwMDAiCiAgICAgICBzdHJva2Utd2lkdGg9IjIuODM0NjUiCiAgICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZmlsbD0ibm9uZSIKICAgICAgIGlkPSJwYXRoODgiIC8+CiAgPC9nPgo8L3N2Zz4K',
            x: 237.5757598876953,
            y: 61.81818389892578,
            type: 'piece',
            piece: {
                id: 'c1',
                type: {
                    name: 'capsule',
                    qualities: [
                        {
                            name: 'entrance',
                            value: 'back',
                            unit: null
                        },
                        {
                            name: 'view',
                            value: 'side'
                        },
                        {
                            name: 'mirrored',
                            value: 'false'
                        }
                    ]
                }
            }
        },
        {
            id: 'c2',
            title: '',
            icon: 'c2',
            x: 250,
            y: 300,
            type: 'piece',
            piece: {
                id: 'c2',
                type: {
                    name: 'capsule',
                    qualities: [
                        {
                            name: 'entrance',
                            value: 'back',
                            unit: null
                        },
                        {
                            name: 'view',
                            value: 'side'
                        },
                        {
                            name: 'mirrored',
                            value: 'true'
                        }
                    ]
                }
            }
        },
        {
            id: 'c3',
            title: '',
            icon: 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAJYAAACWCAYAAAA8AXHiAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAAHYcAAB2HAY/l8WUAABvJSURBVHhe7Z0HkBTFF8YbPUERA4qCCbOeigEMKChgxKxgKMuAigGlBFHLAkoBw59SqwADJZhAObFEKRQDJaiHYEBLFCOYARFQMaKIAWX+/eubPnrnZncn7m3or2rF3dud3en5+r2v33v9pokjISwsEsZ67r8WFonCEssiFVhiWaSCstZYH3zwgZg1a5ZYvHix+PXXX8WOO+4oTjvtNNG+fXv3HRapAWKVG8aMGeO0atWKCeM0bdrUadu2rVNdXe1svvnmTlVVlfp38ODBzi+//OJ+wiJplJXFevXVV8VNN90kXnvtNdGmTRtx6KGHivXWW089/+6778Tee++tHosWLRJLly4VHTt2FFdffbU48sgj3SNYJIWy0Fi4uXvuuUdcdNFF4r///hMzZswQjz32mPjpp5/Eiy++KE444QQxevRosfXWW4vPP/9c9O3bV0yaNElsscUWonfv3uL0008Xr7zyins0i0Sg7FaJAlf2zDPPONLiOFI/OaNGjXLee+8955prrnF22mknR+opZ+bMme67695/yy231P/tqaeeqv887rF///7OwoUL3XdbxEHJEmv27Nn1hOjXr58zb9485+6773Z23nlnp0uXLhmEMqHJ2KNHD1+C8VmeW/0VDyVHLLnCy7BIJimwWiNHjnR+/vln993ZYVo7yDlkyBBlrUyLlo2cFvlRMuIdHVVTUyPuuususcMOOyiRvv7666t/pfsTvXr1EgMGDBDSYrmfCAaO++yzz4pHHnlEbLrppqJnz56iSZMm4umnn1bH3X///a3AjwJFryKGV0dhkXLpqKjIZQm1RbPuMTiKmlimjkJYQyi5+suro6LC6x6lFVTfMWHCBPWadY/BUZTEwnpce+21WXUUq780rYfXSvJ9s2bNci6++GKrvwKiqIjFBTVXdrW1tQ2sViHDAfweLGT79u193SMWjdWoRUMUBbH8LEQaOioq3n///QxrZRJMintn/PjxqVrQUkSjEytqPKrQMMnv1Xzaoln3uA6NRqwgq7Ag8ahCwySYtlZWfzVEwYnFhfFaJNNqlYpu0edxwAEH+E4MrG8lp4cKRiyvK8Ei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',
            x: 50,
            y: 300,
            type: 'piece',
            piece: {
                id: 'c3',
                type: {
                    name: 'capsule',
                    qualities: [
                        {
                            name: 'entrance',
                            value: 'back',
                            unit: null
                        },
                        {
                            name: 'view',
                            value: 'side'
                        },
                        {
                            name: 'mirrored',
                            value: 'true'
                        }
                    ]
                }
            }
        }
    ],
    edges: [
        {
            source: 'b',
            target: 's',
            type: 'attraction',
            label_from: 'c:0',
            label_to: 'b:t',
            handleTooltipText: 'core(0) to bottom',
            attraction: {
                attracting: {
                    piece: {
                        id: 'b',
                        type: {
                            port: {
                                specifiers: [
                                    {
                                        context: 'core',
                                        group: '0'
                                    }
                                ]
                            }
                        }
                    }
                },
                attracted: {
                    piece: {
                        id: 's',
                        type: {
                            port: {
                                specifiers: [
                                    {
                                        context: 'bottom',
                                        group: 'true'
                                    }
                                ]
                            }
                        }
                    }
                }
            }
        },
        {
            source: 's',
            target: 'cp',
            type: 'attraction',
            label_from: 't:t',
            label_to: 'b:t',
            handleTooltipText: 'top to bottom',
            attraction: {
                attracting: {
                    piece: {
                        id: 's',
                        type: {
                            port: {
                                specifiers: [
                                    {
                                        context: 'top',
                                        group: 'true'
                                    }
                                ]
                            }
                        }
                    }
                },
                attracted: {
                    piece: {
                        id: 'cp',
                        type: {
                            port: {
                                specifiers: [
                                    {
                                        context: 'bottom',
                                        group: 'true'
                                    }
                                ]
                            }
                        }
                    }
                }
            }
        }
    ]
}

interface FormationEditorProps {
    className?: string
    onPieceEdit: (piece: PieceInput) => PieceInput
    onAttractionEdit: (attraction: AttractionInput) => AttractionInput
}

const FormationEditor = forwardRef((props: FormationEditorProps, ref) => {
    const [graph, setGraph] = useState(sample)
    const [selected, setSelected] = useState<SelectionT | null>(null)
    const [copiedNodes, setCopiedNodes] = useState<INode[]>([])
    const [copiedEdges, setCopiedEdges] = useState<IEdge[]>([])
    const graphViewRef = useRef(null)

    const getDraft = (): IDraft => {
        return {
            nodes: graph.nodes,
            edges: graph.edges
        }
    }

    const zoomToFit = () => {
        if (graphViewRef.current) {
            graphViewRef.current.handleZoomToFit()
        }
    }

    useImperativeHandle(ref, () => ({
        zoomToFit,
        getDraft
    }))

    const onSelect = (newSelection: SelectionT, event?: any): void => {
        // when only an edge is selected, then the attraction editor should be opened
        if (!newSelection.nodes && newSelection.edges?.size === 1) {
            const edge = newSelection.edges.values().next().value
            const sourceNode = graph.nodes.find((node) => node.id === edge.source)
            const targetNode = graph.nodes.find((node) => node.id === edge.target)
            if (sourceNode && targetNode) {
                const attraction = edge.attraction
                const editedAttraction = props.onAttractionEdit(attraction)
                if (editedAttraction) {
                    const newEdge = {
                        ...edge,
                        attraction: editedAttraction
                    }
                    setGraph({
                        ...graph,
                        edges: graph.edges.map((e) => (e === edge ? newEdge : e))
                    })
                }
            }
            return
        }

        // alt key for node edit mode
        if (event && event.altKey === true) {
            const selectedNode = graph.nodes.find(
                (node) => node.id === newSelection?.nodes?.keys().next().value
            )
            if (selectedNode.type === 'piece') {
                const piece = props.onPieceEdit(selectedNode.piece)
                if (piece) {
                    setGraph({
                        ...graph,
                        nodes: graph.nodes.map((node) => {
                            if (node.id === selectedNode.id) {
                                return {
                                    ...node,
                                    piece
                                }
                            }
                            return node
                        })
                    })
                }
            }
            return
        }

        // event is only active when clicked on a node
        if (event == null && !newSelection.nodes && !newSelection.edges) {
            setSelected(null)
            return
        }
        // Remove the previously selected nodes and edges from the selectionState if they are in the new selection.
        // Add the new selected nodes and edges if they were not in the previous selection.
        const newNodes = new Map(selected?.nodes)
        if (newSelection.nodes) {
            newSelection.nodes.forEach((node, nodeId) => {
                if (GraphUtils.isEqual(node, selected?.nodes?.get(nodeId))) {
                    newNodes.delete(nodeId)
                } else {
                    newNodes.set(nodeId, node)
                }
            })
        }
        const newEdges = new Map(selected?.edges)
        if (newSelection.edges) {
            newSelection.edges.forEach((edge, edgeId) => {
                if (
                    selected?.edges &&
                    [...selected.edges.values()].some((selectedEdge) =>
                        GraphUtils.isEqual(selectedEdge, edge)
                    )
                ) {
                    newEdges.delete(edgeId)
                } else {
                    newEdges.set(edgeId, edge)
                }
            })
        }
        // check for orphaned edges
        newEdges?.forEach((edge, edgeId) => {
            selected?.nodes?.forEach((node) => {
                if (node.id === edge.source || node.id === edge.target) {
                    newEdges.delete(edgeId)
                }
            })
        })
        setSelected({ nodes: newNodes, edges: newEdges })
    }

    const onCreateNode = (x: number, y: number, event: any): void => {
        const id = Generator.generateRandomId(x + y)

        const newNode = {
            id,
            type: 'piece',
            icon: '',
            x,
            y
        }

        setGraph({
            ...graph,
            nodes: [...graph.nodes, newNode]
        })
    }

    const onUpdateNode = (
        node: INode,
        updatedNodes?: Map<string, INode> | null,
        updatedNodePosition?: IPoint
    ): void | Promise<any> => {}

    const onCreateEdge = (sourceNode: INode, targetNode: INode): void => {
        const newEdge = {
            source: sourceNode.id,
            target: targetNode.id,
            type: 'attraction'
        }

        setGraph({
            ...graph,
            edges: [...graph.edges, newEdge]
        })
    }

    const onDeleteSelected = (selected: SelectionT) => {
        const newNodes = graph.nodes.filter((node) => !selected.nodes?.has(node.id))
        const newEdges = graph.edges.filter((edge) => {
            return (
                !selected.nodes?.has(edge.source.toString()) &&
                !selected.nodes?.has(edge.target.toString())
            )
        })

        setGraph({
            nodes: newNodes,
            edges: newEdges
        })
    }

    const onCopySelected = () => {
        if (selected && selected.nodes) {
            const nodesToCopy = graph.nodes.filter((node) => selected.nodes?.has(node.id))
            const topLeftNode = nodesToCopy.reduce((prev, curr) => ({
                x: Math.min(prev.x, curr.x),
                y: Math.min(prev.y, curr.y)
            }))
            const nodesWithRelativePositions = nodesToCopy.map((node) => ({
                ...node,
                x: node.x - topLeftNode.x,
                y: node.y - topLeftNode.y
            }))
            setCopiedNodes(nodesWithRelativePositions)

            const edgesToCopy = graph.edges.filter(
                (edge) => selected.nodes?.has(edge.source) && selected.nodes?.has(edge.target)
            )
            setCopiedEdges(edgesToCopy)
        }
    }

    const onPasteSelected = (selected?: SelectionT | null, xyCoords?: IPoint): void => {
        if (copiedNodes.length > 0 && xyCoords) {
            const idMap = new Map()
            const newNodes = copiedNodes.map((node) => {
                const newId = Generator.generateRandomId(xyCoords.x + node.x + xyCoords.y + node.y)
                idMap.set(node.id, newId)
                return {
                    ...node,
                    id: newId,
                    x: xyCoords.x + node.x,
                    y: xyCoords.y + node.y
                }
            })
            const newEdges = copiedEdges.map((edge) => ({
                ...edge,
                source: idMap.get(edge.source),
                target: idMap.get(edge.target)
            }))
            setGraph({
                ...graph,
                nodes: [...graph.nodes, ...newNodes],
                edges: [...graph.edges, ...newEdges]
            })
        }
    }

    const onSwapEdge = (sourceNode: INode, targetNode: INode, edge: IEdge): void => {
        const newEdge = {
            source: sourceNode.id,
            target: targetNode.id,
            type: edge.type
        }

        setGraph({
            ...graph,
            edges: graph.edges.map((e) => (e === edge ? newEdge : e))
        })
    }

    const canSwapEdge = (
        sourceNode: INode,
        hoveredNode: INode | null,
        swapEdge: IEdge
    ): boolean => {
        return true
    }

    const onContextMenu = (x: number, y: number, event: any): void => {}

    const renderSvg = (
        svgString: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        return (
            <foreignObject x="-16" y="-16" width="32" height="32">
                <SVG
                    className={`cursor-pointer ${isSelected ? 'opacity-50' : 'opacity-100'}`}
                    src={svgString}
                    width="32"
                    height="32"
                />
            </foreignObject>
        )
    }

    const renderImage = (
        imageData: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        return (
            <foreignObject x="-19" y="-19" width="38" height="38">
                <Avatar
                    className={`cursor-pointer ${isSelected ? 'opacity-50' : 'opacity-100'}`}
                    src={imageData}
                    size={38}
                ></Avatar>
            </foreignObject>
        )
    }

    const renderText = (
        text: string,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        const className = `node-text ${isSelected ? 'selected' : ''}`
        return (
            <text className={className} textAnchor="middle" dy=".3em">
                {text}
            </text>
        )
    }

    const renderNodeText = (
        data: any,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        const [iconData, iconKind] = getIconData(data.icon)

        switch (iconKind) {
            case IconKind.Svg:
                return renderSvg(iconData, id, isSelected)
            case IconKind.Image:
                return renderImage(iconData, id, isSelected)
            case IconKind.Text:
                return renderText(iconData, id, isSelected)
        }
    }

    const nodes = graph.nodes
    const edges = graph.edges

    const NodeTypes = GraphConfig.NodeTypes
    const NodeSubtypes = GraphConfig.NodeSubtypes
    const EdgeTypes = GraphConfig.EdgeTypes

    return (
        <div id="formation-editor" className={props.className + ' h-screen w-screen'}>
            <GraphView
                ref={graphViewRef}
                nodeKey={NODE_KEY}
                nodes={nodes}
                edges={edges}
                selected={selected}
                nodeTypes={NodeTypes}
                nodeSubtypes={NodeSubtypes}
                edgeTypes={EdgeTypes}
                // layoutEngineType='VerticalTree'
                allowMultiselect={true}
                gridSpacing={20}
                gridDotSize={0}
                nodeSize={100}
                edgeHandleSize={200}
                edgeArrowSize={4}
                // rotateEdgeHandle={false}
                minZoom={0.1}
                maxZoom={4}
                showGraphControls={false}
                canSwapEdge={canSwapEdge}
                onSwapEdge={onSwapEdge}
                onArrowClicked={(selectedEdge: IEdge): void => {}}
                onSelect={onSelect}
                onCreateNode={onCreateNode}
                onUpdateNode={onUpdateNode}
                onCreateEdge={onCreateEdge}
                onDeleteSelected={onDeleteSelected}
                onCopySelected={onCopySelected}
                onPasteSelected={onPasteSelected}
                onContextMenu={onContextMenu}
                renderNodeText={renderNodeText}
            ></GraphView>
        </div>
    )
})

FormationEditor.displayName = 'FormationEditor'

type TransferItem = GetProp<TransferProps, 'dataSource'>[number]
type TableRowSelection<T extends object> = TableProps<T>['rowSelection']

interface QualityTransferProps extends TransferProps<TransferItem> {
    dataSource: Quality[]
}

const QualityTransfer = ({ ...restProps }: QualityTransferProps): JSX.Element => (
    <Transfer
        rowKey={(record) => record.name}
        showSelectAll={false}
        locale={{
            itemUnit: 'quality',
            itemsUnit: 'qualities',
            searchPlaceholder: 'Search qualities',
            notFoundContent: 'Quality not found',
            remove: 'Remove quality',
            selectAll: 'Select all qualities',
            removeAll: 'Remove all qualities',
            removeCurrent: 'Remove current quality',
            selectCurrent: 'Select current quality',
            selectInvert: 'Invert quality selection',
            titles: ['Available', 'Selected']
        }}
        {...restProps}
    >
        {({
            filteredItems,
            onItemSelectAll,
            onItemSelect,
            selectedKeys: listSelectedNames,
            disabled: listDisabled
        }) => {
            const rowSelection: TableRowSelection<TransferItem> = {
                getCheckboxProps: (item) => ({ disabled: listDisabled || item.disabled }),
                onSelectAll(selected, selectedRows) {
                    const treeSelectedNames = selectedRows
                        .filter((item) => !item.disabled)
                        .map(({ name }) => name)
                    const diffNames = selected
                        ? difference(treeSelectedNames, listSelectedNames)
                        : difference(listSelectedNames, treeSelectedNames)
                    onItemSelectAll(diffNames as string[], selected)
                },
                onSelect({ name }, selected) {
                    onItemSelect(name as string, selected)
                },
                onSelectInvert() {
                    const diffNames = difference(
                        filteredItems.map(({ name }) => name),
                        listSelectedNames
                    )
                    onItemSelectAll(diffNames, true)
                },
                selectedRowKeys: listSelectedNames
            }

            return (
                <Table
                    rowSelection={rowSelection}
                    columns={[
                        {
                            dataIndex: 'name',
                            title: 'Name'
                        },
                        {
                            dataIndex: 'value',
                            title: 'Value',
                            render: (value, record) => <Tag>{`${value} ${record.unit || ''}`}</Tag>
                        }
                    ]}
                    dataSource={filteredItems}
                    size="small"
                    style={{ pointerEvents: listDisabled ? 'none' : undefined }}
                    onRow={({ name, disabled: itemDisabled }) => ({
                        onClick: () => {
                            if (itemDisabled || listDisabled) return
                            onItemSelect(
                                name as string,
                                !listSelectedNames.includes(name as string)
                            )
                        }
                    })}
                    locale={{
                        emptyText: 'No qualities'
                    }}
                />
            )
        }}
    </Transfer>
)

function App(): JSX.Element {
    const [kit, setKit] = useState<Kit | null>(null)
    const [isPieceEditorOpen, setIsPieceEditorOpen] = useState(false)
    const [piece, setPiece] = useState<PieceInput | null>(null)
    const [isAttractionEditorOpen, setIsAttractionEditorOpen] = useState(false)
    const [Attraction, setAttraction] = useState<AttractionInput | null>(null)
    const selectDirectoryIpcHandle = (): void =>
        window.electron.ipcRenderer.send('select-directory')
    const saveDraftIpcHandle = (draft: IDraft): void =>
        window.electron.ipcRenderer.send('save-draft', JSON.stringify(draft))
    const formationEditorRef = useRef(null)
    const actions = [
        {
            id: 'open-kit',
            name: 'Open Kit',
            shortcut: ['$mod+o'],
            keywords: 'new',
            section: 'Files',
            perform: () => {
                selectDirectoryIpcHandle()
                window.electron.ipcRenderer.on('directory-selected', (event, path) => {
                    // not implemented yet
                    // setKit(kit)
                })
            }
        },
        {
            id: 'reload-kit',
            name: 'Reload Kit',
            shortcut: ['$mod+r'],
            keywords: 'update',
            section: 'Files',
            perform: () => {}
        },
        {
            id: 'save-draft',
            name: 'Save draft',
            shortcut: ['$mod+s'],
            keywords: 'store session',
            section: 'Files',
            perform: () => {
                if (formationEditorRef.current) {
                    saveDraftIpcHandle(formationEditorRef.current.getDraft())
                    window.electron.ipcRenderer.on('draft-saved', (event, path) => {
                        // not implemented yet
                    })
                }
            }
        },
        {
            id: 'zoom-to-fit',
            name: 'Zoom to Fit',
            shortcut: ['$mod+t'],
            keywords: 'formation',
            section: 'Navigation',
            perform: () => {
                if (formationEditorRef.current) {
                    formationEditorRef.current.zoomToFit()
                }
            }
        }
    ]

    const showPieceEditor = () => {
        setIsPieceEditorOpen(true)
    }

    const savePieceEditor = (piece: PieceInput) => {
        setPiece(piece)
        setIsPieceEditorOpen(false)
    }

    const handleCancelPiece = () => {
        setIsPieceEditorOpen(false)
    }

    const onPieceEdit = (piece: PieceInput): PieceInput => {
        setPiece(piece)
        showPieceEditor()
        return piece
    }

    const showAttractionEditor = () => {
        setIsAttractionEditorOpen(true)
    }

    const handleSaveAttraction = () => {
        setIsAttractionEditorOpen(false)
    }

    const handleCancelAttraction = () => {
        setIsAttractionEditorOpen(false)
    }

    const onAttractionEdit = (attraction: AttractionInput): AttractionInput => {
        setAttraction(attraction)
        showAttractionEditor()
        return attraction
    }

    const isEditorActive = (): boolean => {
        return !isPieceEditorOpen && !isAttractionEditorOpen
    }

    // const mockdata = [
    //     {
    //         name: 'entrance',
    //         value: 'back',
    //         unit: null
    //     },
    //     {
    //         name: 'view',
    //         value: 'side'
    //     },
    //     {
    //         name: 'mirrored',
    //         value: 'true'
    //     },
    //     {
    //         name: 'length',
    //         value: '420',
    //         unit: 'cm'
    //     }
    // ]

    // const originTargetNames = ['view']

    const [targetNames, setTargetNames] = useState<string[]>()

    const onChange = (nextTargetNames: string[], direction: string, moveNames: string[]) => {
        setTargetNames(nextTargetNames)
    }

    return (
        <ConfigProvider
            locale={enUS}
            theme={{
                algorithm: [theme.darkAlgorithm],
                token: {
                    colorPrimary: '#ff344f',
                    colorBgBase: '#002430',
                    colorTextBase: '#f7f3e3',
                    colorError: '#a60009',
                    colorWarning: '#fccf05',
                    colorInfo: '#dbbea1',
                    colorSuccess: '#7eb77f',
                    colorInfoHover: '#ff344f',
                    colorBgSpotlight: '#ff344f',
                    fontFamily: 'Anta, sans-serif',
                    boxShadow: 'none',
                    boxShadowSecondary: 'none',
                    boxShadowTertiary: 'none',
                    wireframe: false
                }
            }}
        >
            <KBarProvider actions={actions}>
                <div className="font-sans bg-light dark:bg-dark">
                    <CommandBar />
                    <FormationEditor
                        className={isEditorActive() ? '' : 'blur-sm'}
                        ref={formationEditorRef}
                        onPieceEdit={onPieceEdit}
                        onAttractionEdit={onAttractionEdit}
                    />
                </div>
                <ModalForm<{ piece: PieceInput }>
                    title="Type"
                    open={isPieceEditorOpen}
                    autoFocusFirstInput
                    modalProps={{
                        destroyOnClose: true,
                        onCancel: handleCancelPiece
                    }}
                    onFinish={async (values) => {
                        savePieceEditor(piece)
                        return true
                    }}
                    submitter={{
                        searchConfig: {
                            submitText: 'Save',
                            resetText: 'Cancel'
                        },
                        resetButtonProps: {
                            onClick: handleCancelPiece
                        }
                    }}
                >
                    <ProFormSelect
                        // request={async () => [
                        //     { value: 'capsule', label: 'capsule' },
                        //     { value: 'shaft', label: 'shaft' }
                        // ]}
                        options={[
                            { value: 'capsule', label: 'capsule' },
                            { value: 'shaft', label: 'shaft' }
                          ]}
                        name="name"
                        label="Name"
                        placeholder="Please select a type"
                    />
                    <QualityTransfer
                        dataSource={piece?.type?.qualities || []}
                        targetKeys={piece?.type?.qualities?.map((quality) => quality.name) || []}
                        onChange={onChange}
                        filterOption={(inputValue, item) =>
                            item.name.indexOf(inputValue) !== -1 ||
                            item.value.indexOf(inputValue) !== -1 ||
                            (item.unit && item.unit.indexOf(inputValue) !== -1)
                        }
                        showSearch={true}
                        disabled={false}
                    />
                </ModalForm>
                <Modal
                    title="Attraction Editor"
                    open={isAttractionEditorOpen}
                    onOk={handleSaveAttraction}
                    onCancel={handleCancelAttraction}
                >
                    <p>
                        Edit port type specifiers here... Can you imagine a threejs viewer where you
                        can pick your port without knowing any specifiers?😱
                    </p>
                </Modal>
            </KBarProvider>
        </ConfigProvider>
    )
}
export default App
