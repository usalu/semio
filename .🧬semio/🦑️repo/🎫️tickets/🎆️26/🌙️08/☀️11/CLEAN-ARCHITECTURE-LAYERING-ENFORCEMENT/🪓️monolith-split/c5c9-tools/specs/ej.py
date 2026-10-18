from specs.lib import *
EJ = LY + '/⚙️engine/📤️export/🔣️json'
H = 251
B = [546]
CH = [
    ch('🧾', 'limits-and-string-sources', 'limits_and_string_sources', [(23, 131)],
       'Limits and string sources of the layout export JSON: the node, string, item and fragment caps, the top array, record owner and string array kinds, the string source and the string write cursor.', glob=False),
    ch('🌳', 'typed-json-nodes-and-scalars', 'typed_json_nodes_and_scalars', [(133, 228), (240, 249), (1419, 1459)],
       'Typed JSON nodes and scalars of the layout export JSON: the typed node tree, the typed cursor record, the owned byte accounting, the scalar trait with its bool, float, option and vector impls and the finite number writer.'),
    ch('🏗', 'cursor-construction-and-owner-pages', 'cursor_construction_and_owner_pages', [(251, 546), (1362, 1409)],
       'Cursor construction and owner pages of the layout export JSON: the cursor constructors, string resolution with escapes, output and node accounting, the advance step, the owner page and the frame string fields and bounds.'),
    ch('📖', 'expand-document', 'expand_document', [(548, 1360)],
       'Expand document of the layout export JSON: the bounded expansion of one typed node into output bytes for the document, page, frame, story, style, layer and package fragments.'),
]
SPEC = dict(orig='ej.rs', file=EJ + '/🦀️.rs', children=CH, pre=split_impl_ln(H, B), expect_extra=split_extra_ln('ej.rs', H, B),
            external=mounted(EJ + '/🦀️.rs', orig='ej.rs'))
