from specs.lib import *
RC = '🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract'
CH = [
    ch('📜', 'clone-and-cursor-traits', 'clone_and_cursor_traits', [(24, 133)],
       'Clone and cursor traits of the retained clone contract: the retained clone trait with its bitwise hooks, the bulk page bytes, the bulk run helpers, the clone cursor trait, the close record and the binding close helper.'),
    ch('🔢', 'scalar-cursor', 'scalar_cursor', [(135, 236)],
       'Scalar cursor of the retained clone contract: the scalar cursor, its cursor implementation and the scalar retained clone macro with its invocation for the primitive types.'),
    ch('🧮', 'array-cursor', 'array_cursor', [(238, 379)],
       'Array cursor of the retained clone contract: the fixed array cursor with its default, cursor and retained clone implementations.'),
    ch('🔤', 'string-cursor', 'string_cursor', [(381, 501)],
       'String cursor of the retained clone contract: the string cursor with its cursor implementation and the string retained clone implementation.'),
    ch('📚', 'vec-cursor', 'vec_cursor', [(503, 706)],
       'Vec cursor of the retained clone contract: the vector cursor with its default, constructor, cursor and retained clone implementations.'),
    ch('🎁', 'option-cursor', 'option_cursor', [(708, 837)],
       'Option cursor of the retained clone contract: the option cursor with its default, cursor and retained clone implementations.'),
    ch('🗃', 'box-cursor', 'box_cursor', [(839, 987)],
       'Box cursor of the retained clone contract: the box cursor with its default, cursor and retained clone implementations.'),
    ch('🧱', 'tuple-cursors', 'tuple_cursors', [(989, 1125)],
       'Tuple cursors of the retained clone contract: the tuple retained clone macro and its invocations for the pair and triple cursors.'),
]
SPEC = dict(orig='rcc.rs', file=RC + '/🦀️.rs', children=CH, external=mounted(RC + '/🦀️.rs', orig='rcc.rs'))
