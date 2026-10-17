from specs.lib import *

CH = [
    ch('🔖', 'limits-and-codes', 'limits_and_codes', [(34, 93)],
       'Limits and stable refusal codes of the time-travel session: the text ceiling, the fault and refusal code constants and their two classifiers.', glob=False),
    ch('🪪', 'identity-and-session', 'identity_and_session', [(97, 241)],
       'Identity and session state of the time-travel session: the base stamp and target, the stage machine, the draft, pending and progress records, the review verdicts and the session itself.'),
    ch('📨', 'events', 'events', [(245, 413)],
       'Events of the time-travel session: the choice, the event and its stable key with their constructors, accessors and wire conversions.'),
    ch('🎬', 'effects-and-refusals', 'effects_and_refusals', [(417, 535)],
       'Effects and refusals of the time-travel session: the effect and its kind, the refusal enum with its code and display, and the error implementation.'),
    ch('⚙', 'reducer', 'reducer', [(539, 872)],
       'Reducer of the time-travel session: the single implementation block that applies one event to the session and returns the effects or the refusal.'),
    ch('🌍', 'labels', 'labels', [(876, 1075)],
       'Labels of the time-travel session: the code label table, the label enum and its English and German texts.'),
]

SPEC = dict(
    orig='tt.rs',
    file=TT + '/🦀️.rs',
    children=CH,
    external=mounted(TT + '/🦀️.rs', TT + '/📦️packages/🦀️rust/🦀️.rs'),
)
