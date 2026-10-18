from specs.lib import *
PJ = '🧰️framework/🔨️modules/🎒️pack/🔤️json'
CH = [
    ch('🚨', 'json-error', 'json_error', [(43, 99)],
       'JSON error: every parse failure with its byte offset, the nesting depth limit and the error conversions.'),
    ch('🔢', 'number', 'number', [(101, 193)],
       'JSON number: the integer and float number carrier with its comparisons and numeric conversions.', glob=False),
    ch('🧱', 'object-and-value', 'object_and_value', [(195, 582)],
       'JSON object and value: the ordered object, the value enum, its indexing, comparisons and conversions, and the order-insensitive equality.'),
    ch('🌉', 'dsl-value-bridge', 'dsl_value_bridge', [(584, 644)],
       'DSL value bridge: the conversions between JSON values and domain-neutral DSL values.'),
    ch('🔤', 'lexer', 'lexer', [(653, 858)],
       'Lexer: the token enum, the token-at-a-time lexer and the shared number scan.'),
    ch('🧮', 'parser-and-grammar-cursor', 'parser_and_grammar_cursor', [(860, 1249)],
       'Parser and grammar cursor: whole-document parsing, the retained grammar cursor with its frames and the value projection with its retirement owners.'),
    ch('✍', 'text-writer', 'text_writer', [(1251, 1403)],
       'Text writer: compact and pretty serialization of values, strings and numbers.'),
    ch('🎯', 'float-writer', 'float_writer', [(1404, 1800)],
       'Float writer: the scalar text sink, the f64 and f32 spellings and the exact-arithmetic digit generator.'),
    ch('🎛', 'controlled-and-borrowed-bridge', 'controlled_and_borrowed_bridge', [(1802, 2136)],
       'Controlled and borrowed bridge: the controlled string conversions, the native write nodes and cursors and the controlled reader and writer.'),
]
SPEC = dict(orig='pj.rs', file=PJ + '/🦀️.rs', children=CH, wiring='block', external=mounted(PJ + '/🦀️.rs', orig='pj.rs'))
