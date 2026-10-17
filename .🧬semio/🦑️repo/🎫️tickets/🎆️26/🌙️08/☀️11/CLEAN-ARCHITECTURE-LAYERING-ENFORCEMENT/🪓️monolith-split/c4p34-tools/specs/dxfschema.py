from specs.lib import *

CH = [
    ch('🧮', 'collection-algebra-core', 'collection_algebra_core', [(46, 290)],
       'Collection algebra of the DXF diff: the index-keyed and name-keyed element traits with their generic apply, inverse and absorb-pair cores.'),
    ch('🏷', 'header-and-layer-diffs', 'header_and_layer_diffs', [(294, 521)],
       'Header variable and layer diffs of the DXF diff: the per-element diffs, their modified and added records and the name-keyed collection diffs.'),
    ch('🎨', 'style-linetype-and-tables-diffs', 'style_linetype_and_tables_diffs', [(525, 821)],
       'Style, linetype and table diffs of the DXF diff: the name-keyed style and linetype collection diffs and the tables diff that combines them.'),
    ch('📐', 'entity-diff-records', 'entity_diff_records', [(825, 943), (1335, 1396)],
       'Entity diff records of the DXF diff: the per-kind entity diffs, the entity diff enum and the modified, added and collection records of the entities.'),
    ch('🔧', 'entity-diff-apply', 'entity_diff_apply', [(945, 1333)],
       'Entity diff application of the DXF diff: the emptiness and field-change helpers, the inverse and the apply functions and the index element behaviour of an entity.'),
    ch('🧱', 'block-diffs', 'block_diffs', [(1400, 1524)],
       'Block diffs of the DXF diff: the per-block diff, its index element behaviour, the modified and added records and the block collection diff.'),
    ch('🎯', 'dxf-diff-root', 'dxf_diff_root', [(1528, 1791)],
       'Root diff of the DXF artifact: the sparse diff record, the target validation, the unchecked apply and the mutation and diff-algebra implementations.'),
    ch('🛠', 'diff-builders', 'diff_builders', [(1796, 1903)],
       'Builders of the DXF diff: the per-collection field-change helpers and the one-edit diff constructors the mutations use.'),
]

SPEC = dict(
    orig='dxfschema.rs.txt',
    file=DXFS + '/🦀️.rs',
    nest=False,
    keep_empty=('Primitives', 'GeometryCodecs', 'ValueCodecs', 'ItemCodecs', 'CollectionTripleCodecs', 'ItemBinaryCodecs', 'DiffBinaryCodecs', 'CollectionTripleBinaryCodecs', 'TopLevel', '🧪️Tests'),
    external=sub(DXFS, skip=folders(CH)),
    outside=outside('dxfschema'),
    children=CH,
)


FACET_NEW = '            rust: concat!(' + ', '.join(['include_str!("🔺️diff/🦀️.rs")'] + ['include_str!("🔺️diff/%s/🦀️.rs")' % c['folder'] for c in CH]) + '),\n'
SPEC['facet'] = dict(file=DXFS.rsplit('/', 1)[0] + '/🦀️.rs', old='            rust: include_str!("🔺️diff/🦀️.rs"),\n', new=FACET_NEW)
