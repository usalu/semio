from specs.lib import *

CH = [
    ch('📦', 'primitives-and-values', 'primitives_and_values', [(10, 132)],
       'Binary primitives of the DXF diff codec: the length-prefixed bytes and strings, the optional presence byte, the floats and points and the DXF value pair.'),
    ch('📐', 'group-codes-and-entities', 'group_codes_and_entities', [(134, 351)],
       'Group codes and entities of the DXF diff codec: the group-code and vertex pair codecs and the entity and entity collection pair codecs.'),
    ch('🗂', 'tables-and-blocks', 'tables_and_blocks', [(353, 438)],
       'Table items and blocks of the DXF diff codec: the header variable, layer, style, linetype and block pair codecs.'),
    ch('🏷', 'item-diffs', 'item_diffs', [(440, 673)],
       'Item diffs of the DXF diff codec: the header variable, layer, style, linetype, entity, block and tables diff pair codecs.'),
    ch('🧵', 'triples-and-collection-diffs', 'triples_and_collection_diffs', [(675, 838)],
       'Collection triples of the DXF diff codec: the name-keyed and index-keyed removed, modified and added triple codecs and the collection diff pair codecs.'),
    ch('🚦', 'diff-binary-impl', 'diff_binary_impl', [(840, 910)],
       'Frame of the DXF diff codec: the binary diff implementation of the root diff.'),
]

SPEC = dict(
    orig='dxfbin.rs.txt',
    file=DXFB + '/🦀️.rs',
    nest=False,
    children=[],
    expect_missing=['mod diff_codec {', '}'],
    inner=dict(
        mod='diff_codec',
        indent=0,
        folder='🧬️diff-codec',
        doc='//! 🧬️ Binary codec of the DXF diff: the varint-framed primitives, the item pair codecs, the collection triple codecs and the binary diff implementation of the root diff, with the shared imports of the former inline module. See [`super`].',
        nest=False,
        children=CH,
    ),
)
