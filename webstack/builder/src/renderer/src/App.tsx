import './App.scss'
import {
    useState,
    forwardRef,
    Ref,
    useMemo,
    Fragment,
    Suspense,
    useEffect
} from 'react'
import { KBarProvider } from 'kbar'
import {
    AttractionInput,
    Formation,
    Kit,
    Piece,
    PieceInput,
    Representation
} from '@renderer/semio'

import { KBarAnimator, KBarPortal, KBarPositioner, KBarSearch } from 'kbar'
import { KBarResults, useMatches } from 'kbar'
import { ActionId, ActionImpl } from 'kbar'
import {
    ConfigProvider,
    theme
} from 'antd'
import enUS from 'antd/lib/calendar/locale/en_US'
import { Canvas, useLoader } from '@react-three/fiber'
import {
    OrbitControls,
    useGLTF,
    Select,
    GizmoHelper,
    GizmoViewport
} from '@react-three/drei'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'

import tailwindConfig from '../../../tailwind.config.js';

const { theme: { extend: { colors } } } = tailwindConfig;

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



interface RepresentationThreeProps {
    representation: Representation
}

const RepresentationThree = ({ representation }: RepresentationThreeProps) => {
    const { nodes } = useLoader(GLTFLoader, representation.url)
    return (
        <group>
            {Object.values(nodes).map((node, i) => (
                <primitive
                    key={i}
                    object={node}
                    attach={(parent, self) => {
                        parent.add(self)
                        return () => parent.remove(self)
                    }}
                />
            ))}
        </group>
    )
}

RepresentationThree.displayName = 'Representation'

interface PieceThreeProps {
    piece: Piece
}

const PieceThree = ({ piece }: PieceThreeProps) => {
    const [lod, setLod] = useState('')
    const [tags, setTags] = useState([''])
    return (
        <RepresentationThree
            representation={
                piece.type.representations.find(
                    (representation) =>
                        (representation.lod !== '' || representation.lod === lod) &&
                        (representation.tags.length === 0 ||
                            representation.tags.some((tag) => tags.includes(tag)))
                )!
            }
        />
    )
}

interface FormationThreeProps {
    formation: Formation
}

const FormationThree = ({ formation }: FormationThreeProps) => {
    const { pieces } = formation
    return (
        <group>
            {pieces.map((piece, i) => (
                <PieceThree key={i} piece={piece} />
            ))}
        </group>
    )
}

const kitMock: Kit = {
    name: 'metabolism',
    explanation: 'Everything for metabolistic architecture.',
    icon: '🫀',
    url: 'https://github.com/usalu/semio/examples/metabolism',
    types: [
        {
            name: 'capsule',
            explanation: 'A living capsule for one person.',
            icon: '📦',
            representations: [
                {
                    url: 'representations/capsule_1_1to200_volume_wireframe.glb',
                    lod: '1to200',
                    tags: ['volume']
                },
                {
                    url: 'representations\\capsule_1_1to200_volume.3dm',
                    lod: '1to200',
                    tags: ['volume']
                },
                {
                    url: 'representations\\capsule_1_1to500_volume.3dm',
                    lod: '1to500',
                    tags: ['volume']
                }
            ],
            ports: [
                {
                    plane: {
                        origin: {
                            x: 45,
                            y: -210,
                            z: 0
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'doors',
                            group: 'front'
                        }
                    ]
                }
            ],
            qualities: [
                {
                    name: 'entrance',
                    value: 'back',
                    unit: null
                },
                {
                    name: 'view',
                    value: 'front',
                    unit: null
                },
                {
                    name: 'mirrored',
                    value: 'false',
                    unit: null
                },
                {
                    name: 'length',
                    value: '420',
                    unit: 'cm'
                },
                {
                    name: 'width',
                    value: '260',
                    unit: 'cm'
                },
                {
                    name: 'heigth',
                    value: '250',
                    unit: 'cm'
                },
                {
                    name: 'window shape',
                    value: 'round',
                    unit: null
                }
            ]
        },
        {
            name: 'capital',
            explanation: 'An efficient core with many doors.',
            icon: null,
            representations: [
                {
                    url: 'representations\\capital_1_1to200_volume.3dm',
                    lod: '1to200',
                    tags: ['volume']
                },
                {
                    url: 'representations\\capital_1_1to500_volume.3dm',
                    lod: '1to500',
                    tags: ['volume']
                }
            ],
            ports: [
                {
                    plane: {
                        origin: {
                            x: 0,
                            y: 0,
                            z: 0
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'bottom',
                            group: 'true'
                        }
                    ]
                }
            ],
            qualities: [
                {
                    name: 'peak',
                    value: 'south west',
                    unit: null
                },
                {
                    name: 'length',
                    value: '550',
                    unit: 'cm'
                },
                {
                    name: 'width',
                    value: '550',
                    unit: 'cm'
                },
                {
                    name: 'height',
                    value: '1150',
                    unit: 'cm'
                },
                {
                    name: 'height side',
                    value: '551',
                    unit: 'cm'
                },
                {
                    name: 'height low',
                    value: '100',
                    unit: 'cm'
                },
                {
                    name: 'wall thickness',
                    value: '30',
                    unit: 'cm'
                }
            ]
        },
        {
            name: 'base',
            explanation:
                'A two storey office building with a setback and a colonade on the groundfloor towards the street.',
            icon: '🏫',
            representations: [
                {
                    url: 'representations\\base_1_1to200_volume.3dm',
                    lod: '1to200',
                    tags: ['volume']
                },
                {
                    url: 'representations\\base_1_1to500_volume.3dm',
                    lod: '1to500',
                    tags: ['volume']
                }
            ],
            ports: [
                {
                    plane: {
                        origin: {
                            x: -1860,
                            y: -770,
                            z: 750
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'core',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -750,
                            y: -770,
                            z: 750
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'core',
                            group: '1'
                        }
                    ]
                }
            ],
            qualities: [
                {
                    name: 'storeys',
                    value: '2',
                    unit: null
                },
                {
                    name: 'floor height',
                    value: '350',
                    unit: 'cm'
                },
                {
                    name: 'ground floor height',
                    value: '400',
                    unit: 'cm'
                },
                {
                    name: 'cores',
                    value: '2',
                    unit: null
                },
                {
                    name: 'core length',
                    value: '2',
                    unit: 'cm'
                },
                {
                    name: 'core width',
                    value: '2',
                    unit: 'cm'
                }
            ]
        },
        {
            name: 'shaft',
            explanation: 'An efficient core with many doors.',
            icon: '🛗',
            representations: [
                {
                    url: 'representations\\shaft_2_1to200_volume.3dm',
                    lod: '1to200',
                    tags: ['volume']
                },
                {
                    url: 'representations\\shaft_2_1to500_volume.3dm',
                    lod: '1to500',
                    tags: ['volume']
                }
            ],
            ports: [
                {
                    plane: {
                        origin: {
                            x: 0,
                            y: 0,
                            z: 0
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'bottom',
                            group: 'true'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 0,
                            y: 0,
                            z: 3300
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'top',
                            group: 'true'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 20
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 20
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 20
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 20
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 111.666664
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 111.666664
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 203.333328
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 203.333328
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '0'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 295
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 295
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 295
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 295
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 386.666656
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 386.666656
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 478.333344
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 478.333344
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '1'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 570
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 570
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 570
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 570
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 661.6667
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 661.6667
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 753.3333
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 753.3333
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '2'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 845
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 845
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 845
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 845
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 936.6667
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 936.6667
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 1028.33337
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 1028.33337
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '3'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 1120
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 1120
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 1120
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 1120
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 1211.66663
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 1211.66663
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 1303.33337
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 1303.33337
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '4'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 1395
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 1395
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 1395
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 1395
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 1486.66663
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 1486.66663
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 1578.33337
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 1578.33337
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '5'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 1670
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 1670
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 1670
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 1670
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 1761.66663
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 1761.66663
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 1853.33337
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 1853.33337
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '6'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 1945
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 1945
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 1945
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 1945
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 2036.66663
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 2036.66663
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 2128.33325
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 2128.33325
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '7'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 2220
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 2220
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 2220
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 2220
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 2311.66675
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 2311.66675
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 2403.33325
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 2403.33325
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '8'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 2495
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 2495
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 2495
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 2495
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 2586.66675
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 2586.66675
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 2678.33325
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 2678.33325
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '9'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: -275,
                            z: 2770
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: -90,
                            z: 2770
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 275,
                            y: 90,
                            z: 2770
                        },
                        xAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        },
                        yAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '2'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: 90,
                            y: 275,
                            z: 2770
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '0'
                        },
                        {
                            context: 'door',
                            group: '3'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: 275,
                            z: 2861.66675
                        },
                        xAxis: {
                            x: 1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: 90,
                            z: 2861.66675
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '1'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -275,
                            y: -90,
                            z: 2953.33325
                        },
                        xAxis: {
                            x: 0,
                            y: 1,
                            z: 0
                        },
                        yAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '0'
                        }
                    ]
                },
                {
                    plane: {
                        origin: {
                            x: -90,
                            y: -275,
                            z: 2953.33325
                        },
                        xAxis: {
                            x: -1,
                            y: 0,
                            z: 0
                        },
                        yAxis: {
                            x: 0,
                            y: -1,
                            z: 0
                        }
                    },
                    specifiers: [
                        {
                            context: 'floor',
                            group: '10'
                        },
                        {
                            context: 'split level',
                            group: '2'
                        },
                        {
                            context: 'door',
                            group: '1'
                        }
                    ]
                }
            ],
            qualities: [
                {
                    name: 'width',
                    value: '550',
                    unit: 'cm'
                },
                {
                    name: 'storeys',
                    value: '11',
                    unit: null
                },
                {
                    name: 'floor height',
                    value: '275',
                    unit: 'cm'
                },
                {
                    name: 'platform',
                    value: 'right',
                    unit: 'cm'
                },
                {
                    name: 'door width',
                    value: '80',
                    unit: 'cm'
                },
                {
                    name: 'door height',
                    value: '200',
                    unit: 'cm'
                },
                {
                    name: 'door offset',
                    value: '115',
                    unit: 'cm'
                }
            ]
        }
    ]
}

function App(): JSX.Element {
    const [kit, setKit] = useState<Kit | null>(kitMock)
    const [draft, setDraft] = useState<IDraft | null>(null)
    const [blobUrls, setBlobUrls] = useState<{ [key: string]: string }>({})
    const [isSelectionBoxActive, setIsSelectionBoxActive] = useState(false)
    const [piecePipette, setPiecePipette] = useState<PieceInput | null>(null)
    const [AttractionPipette, setAttractionPipette] = useState<AttractionInput | null>(null)
    const [transformMode, setTransformMode] = useState<string>('translate')

    // const camera = useResource()

    const actions = [
        {
            id: 'open-kit',
            name: 'Open Kit',
            shortcut: ['$mod+o'],
            keywords: 'new',
            section: 'Files',
            perform: () => {
                window.electron.ipcRenderer.invoke('open-kit').then((kit) => {
                    setKit(kit)
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
                window.electron.ipcRenderer.invoke('save-draft', draft)
            }
        }
    ]

    useEffect(() => {
        if (kit) {
            ;[
                'c:\\git\\semio\\2.x\\examples\\metabolism\\representations\\capsule_1_1to200_volume_wireframe.glb'
            ].forEach((path) => {
                window.electron.ipcRenderer.invoke('get-file-buffer', path).then((buffer) => {
                    const name = 'representations/capsule_1_1to200_volume_wireframe.glb'
                    const blob = new Blob([buffer], { type: 'model/gltf-binary' })
                    const url = URL.createObjectURL(blob)
                    useGLTF.preload(url)
                    setBlobUrls((prev) => ({ ...prev, [name]: url }))
                })
            })
        }
    }, [kit])

    return (
        <div className="h-screen w-screen">
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
                    <CommandBar />
                    <div
                        id="canvas-container"
                        className="relative font-sans bg-light dark:bg-dark flex flex-col h-full"
                    >
                        <Canvas>
                            <Select
                                multiple
                                box
                                border="1px solid #fff"
                                onChange={(selected): void => {
                                    if (!isSelectionBoxActive) {
                                        setIsSelectionBoxActive(true)
                                        console.log('selection starting', selected)
                                    }
                                }}
                                onChangePointerUp={(e) => {
                                    if (isSelectionBoxActive) {
                                        setIsSelectionBoxActive(false)
                                        console.log('selection ending', e)
                                    }
                                }}
                                onClick={(e) => {
                                    console.log('select onClick', e)
                                }}
                            >
                                <Suspense fallback={null}>
                                    <RepresentationThree
                                        representation={{
                                            url: blobUrls[
                                                'representations/capsule_1_1to200_volume_wireframe.glb'
                                            ]
                                        }}
                                    />
                                    <hemisphereLight intensity={0.5} />
                                    <ambientLight intensity={0.5} />
                                </Suspense>
                            </Select>
                            <OrbitControls enabled={!isSelectionBoxActive} />
                            <GizmoHelper
                                alignment="bottom-right" // widget alignment within scene
                                margin={[80, 80]} // widget margins (X, Y)
                                >
                                <GizmoViewport
                                    // axisColors={[colors.primary, colors.secondary, colors.tertiary]}
                                    // labelColor={colors.light} font="Anta"
                                />
                            </GizmoHelper>

                        </Canvas>
                    </div>
                </KBarProvider>
            </ConfigProvider>
        </div>
    )
}
export default App
