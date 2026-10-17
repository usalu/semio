from specs.lib import *

CH = [
    ch('🧩', 'value-tokens-and-parsers', 'value_tokens_and_parsers', [(25, 187)],
       'Value tokens and parsers of the retained record decoder: the role, container and token enums, the varint and UTF-8 readers and the continuation states.'),
    ch('🧭', 'value-cursor', 'value_cursor', [(189, 795)],
       'Value cursor of the retained record decoder: the resumable cursor that admits one wire value per granted step, its allocation and progress records and its drop guard.'),
    ch('📜', 'record-body-cursor', 'record_body_cursor', [(797, 1386)],
       'Record body cursor of the retained record decoder: the body token and phase enums, the resumable record body cursor, its allocation records and its drop guard.'),
]

SPEC = dict(
    orig='dec.rs',
    file=DEC + '/🦀️.rs',
    children=CH,
    external=mounted(DEC + '/🦀️.rs'),
)
