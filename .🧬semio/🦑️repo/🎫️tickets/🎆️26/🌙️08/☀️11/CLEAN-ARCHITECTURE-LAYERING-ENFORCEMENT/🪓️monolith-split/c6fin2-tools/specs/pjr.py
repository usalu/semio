from specs.lib import *
PR = '🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️retained/🧬️schema'
CH = [
    ch('📃', 'decode-pages', 'decode_pages', [(7, 202)],
       'Decode pages: the fixed decode page constants, the page and its faults, the credits, the admission faults and the page ring.'),
    ch('🔡', 'path-and-token-cursor', 'path_and_token_cursor', [(204, 677)],
       'Path and token cursor: the schema path, the token kinds, the lexer states and the token cursor.'),
    ch('🗂', 'record-cursors', 'record_cursors', [(679, 1040)],
       'Record cursors: the field and record specs, the record cursor and the nested record cursor.'),
    ch('🔐', 'string-and-hex-authorities', 'string_and_hex_authorities', [(1043, 1353)],
       'String and hex authorities: the bounded string authority and the hexadecimal authority with their steps.'),
]
SPEC = dict(orig='pjr.rs', file=PR + '/🦀️.rs', children=CH, wiring='block', external=mounted(PR + '/🦀️.rs', orig='pjr.rs'))
