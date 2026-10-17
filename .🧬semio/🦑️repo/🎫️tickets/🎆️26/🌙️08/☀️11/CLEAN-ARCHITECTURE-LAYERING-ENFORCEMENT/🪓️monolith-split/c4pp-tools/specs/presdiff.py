from specs.lib import *


def pre(text):
    a = '//#endregion 🔖️DiffAlgebra\n'
    assert text.count(a) == 1
    return text.replace(a, a + '//#region 🔖️MutationDiffHelpers\n')


CH = [
    ch('🧬', 'diff-types', 'diff_types', [(39, 168)],
       'Presentation diff types: the collection diff aliases, the sparse frame, picture, table, shape, master, layout and slide diff records and the presentation diff.'),
    ch('⚙', 'indexed-and-named-engine', 'indexed_and_named_engine', [(170, 435)],
       'Generic indexed and named engine: apply, inverse, absorb and index transformation over index keyed and name keyed collection triples.'),
    ch('🖼', 'frame-image-and-doc-block-logic', 'frame_image_and_doc_block_logic', [(439, 533)],
       'Frame, image and document block logic: diffing, applying, inverting and absorbing frames, picture images and text box document blocks.'),
    ch('🔷', 'shape-and-table-logic', 'shape_and_table_logic', [(535, 670)],
       'Shape and table logic: applying, inverting and absorbing shape, table row and table cell diffs with their copy variants.'),
    ch('📑', 'master-layout-and-slide-logic', 'master_layout_and_slide_logic', [(672, 751)],
       'Master, layout and slide logic: applying, inverting and absorbing master, layout and slide diffs.'),
    ch('🔌', 'mutation-diff-and-algebra', 'mutation_diff_and_algebra', [(756, 795)],
       'Mutation diff and diff algebra: the mutation diff and diff algebra implementations of the presentation diff.'),
    ch('🛠', 'mutation-diff-helpers', 'mutation_diff_helpers', [(801, 915)],
       'Mutation diff helpers: the diff builders for inserting, removing and setting slides, shapes, masters and layouts.'),
    ch('🌱', 'demo-cases', 'demo_cases', [(1018, 1064)],
       'Demo cases: the two test snapshots and the representative presentation diff cases.', cfg_mod='#[cfg(any(test, feature = "testkit"))]', cfg_use='#[cfg(any(test, feature = "testkit"))]'),
]

SPEC = dict(
    orig='presdiff.rs',
    file=PRESDIFF + '/🦀️.rs',
    pre=pre,
    children=CH,
    wiring='each',
    keep_empty=('Primitives', 'ValueCodecs', 'GenericTripleCodecs', 'DiffValueCodecs', 'TopLevel', 'BinaryPrimitives', 'Tests'),
    external=[PRESDIFF + '/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs'],
)
