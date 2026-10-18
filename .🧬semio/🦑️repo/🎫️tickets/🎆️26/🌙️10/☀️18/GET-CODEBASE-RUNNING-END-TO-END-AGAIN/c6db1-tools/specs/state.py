from specs.lib import *
F = DB + '/🔘️state/🦀️.rs'
CH = [
    ch('🧾', 'canonical-pages-and-retained-state', 'canonical_pages_and_retained_state', [(31, 421)],
       'Canonical pages and retained state: canonical encoding, pages, state entries, the retained state map and the page store.'),
    ch('🗂', 'persistent-map', 'persistent_map', [(425, 706)],
       'Persistent map: the HAMT nodes and `PMap`.'),
    ch('📃', 'persistent-vector-and-text', 'persistent_vector_and_text', [(710, 994)],
       'Persistent vector and text: the vector nodes with `PVec` and the rope with `PText`.'),
    ch('🌳', 'persistent-tree-and-graph', 'persistent_tree_and_graph', [(998, 1357)],
       'Persistent tree and graph: the balanced tree with `PTree` and the persistent graph `PGraph`.'),
    ch('🪟', 'overlay-and-touched-regions', 'overlay_and_touched_regions', [(1361, 1498)],
       'Overlay and touched regions: the base source and overlay root plus the touched region set.'),
]
SPEC = dict(orig='state.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='state.rs'))
