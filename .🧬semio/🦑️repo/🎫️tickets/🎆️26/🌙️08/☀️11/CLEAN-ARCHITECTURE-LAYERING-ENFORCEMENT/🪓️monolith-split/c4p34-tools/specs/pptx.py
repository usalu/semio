from specs.lib import *

CH = [
    ch('🌳', 'xml-tree-parse-and-serialize', 'xml_tree_parse_and_serialize', [(11, 154)],
       'XML tree of the PPTX oracle: the owned element and text node, the quick-xml parser and the serializer.'),
    ch('📦', 'package-and-presentation-model', 'package_and_presentation_model', [(158, 340)],
       'Package and presentation model of the PPTX oracle: the zip package reader and writer, the transform, shape and slide records and the presentation and slide reader.'),
    ch('🖼', 'render-and-write-presentation', 'render_and_write_presentation', [(344, 533)],
       'Presentation writer of the PPTX oracle: the namespace and content-type constants, the shape, slide and relationship renderers and the presentation writer.'),
    ch('🧾', 'json-wire-and-edit-helpers', 'json_wire_and_edit_helpers', [(537, 705)],
       'Wire and edit helpers of the PPTX oracle: the JSON projections, the wire node codec, the node addressing, the vacancy and removal edits and the text and attribute setters.'),
    ch('🧬', 'mutation-apply', 'mutation_apply', [(707, 849)],
       'Mutation application of the PPTX oracle: the kind switch that applies one declared mutation to a parsed package.'),
    ch('🔁', 'entry-points-and-plumbing', 'entry_points_and_plumbing', [(853, 978)],
       'Entry points and plumbing of the PPTX oracle: the apply, inverse, round-trip and projection entry points and the content-type and relationship plumbing.'),
]

SPEC = dict(
    orig='pptx.rs.txt',
    file=PPTX + '/🦀️.rs',
    nest=False,
    children=[],
    expect_missing=['mod oracles {', '}'],
    external=sub(PPTX + '/🧪️tests', keep_root=True),
    inner=dict(
        mod='oracles',
        indent=4,
        folder='🧰️reference-implementation',
        doc='//! 🧰️ Reference implementation of the PPTX oracle: the independent zip and quick-xml composition with the shared imports of the former inline module. See [`super`].',
        nest=False,
        outside=outside('pptx'),
        children=CH,
    ),
)
