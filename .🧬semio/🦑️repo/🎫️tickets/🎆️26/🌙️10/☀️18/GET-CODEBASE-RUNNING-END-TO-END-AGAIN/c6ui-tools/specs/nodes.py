from specs.lib import *
F = U + '/🌳️retained/🌳️tree/🍃️nodes/🦀️.rs'
H = 'impl UiTree {'
B = [716, 1054]
CH = [
    ch('🔖', 'retained-document-tree', 'retained_document_tree', [(11, 174)],
       'Retained document tree: the document tree faults, the page rejection and the `UiDocumentTree` with its node table, lease and surface.'),
    ch('🧩', 'node-model', 'node_model', [(177, 434)],
       'Node model: the node keys, flags, widget specs and state, the layout, paint and accepted layout buckets and the `Node` with its accessors.'),
    ch('🌿', 'ui-tree-core-and-layout', 'ui_tree_core_and_layout', [(436, 716)],
       'Ui tree core and layout: the `UiTree` state, its node access, authored tree items, disclosures, and the accepted, inactive and mounted layout writes.'),
    ch('🪪', 'document-identity-and-composite-rows', 'document_identity_and_composite_rows', [(718, 1057)],
       'Document identity and composite rows: the retained document binding, the interaction transfers, link clearing, child attach and detach and the composite row minting and retirement.'),
    ch('🔗', 'tree-structure-edits', 'tree_structure_edits', [(1059, 1173)],
       'Tree structure edits: child insertion and removal with the sibling links, dirty marking and the children iterator.'),
]
SPEC = dict(orig='nodes.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='nodes.rs'))
