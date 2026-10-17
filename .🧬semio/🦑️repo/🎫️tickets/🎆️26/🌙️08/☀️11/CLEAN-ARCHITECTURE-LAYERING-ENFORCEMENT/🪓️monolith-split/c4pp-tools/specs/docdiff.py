from specs.lib import *


def pre(text):
    assert text.count('//#region 🔖️DocumentDiffLogic') == 1 and not text.rstrip().endswith('//#endregion 🔖️DocumentDiffLogic')
    return text.rstrip('\n') + '\n//#endregion 🔖️DocumentDiffLogic\n'


CH = [
    ch('🧬', 'diff-types', 'diff_types', [(39, 219)],
       'Document diff types: the collection diff aliases, the sparse style, image, run, list, table and block diff records, the block diff enum and the document diff.'),
    ch('⚙', 'indexed-and-named-engine', 'indexed_and_named_engine', [(221, 482)],
       'Generic indexed and named engine: apply, inverse, absorb and index transformation over index keyed and name keyed collection triples.'),
    ch('🖊', 'diff-logic-and-algebra', 'diff_logic_and_algebra', [(486, 987)],
       'Document diff logic and algebra: diffing, applying, inverting and absorbing runs, styles, images and blocks, and the mutation diff and diff algebra implementations of the document diff.', reexport=False),
]

SPEC = dict(
    orig='docdiff.rs',
    file=DOCDIFF + '/🦀️.rs',
    pre=pre,
    children=CH,
    wiring='each',
    external=[DOCDIFF + '/🧪️tests/🔬️unit/🦀️.rs'],
)
