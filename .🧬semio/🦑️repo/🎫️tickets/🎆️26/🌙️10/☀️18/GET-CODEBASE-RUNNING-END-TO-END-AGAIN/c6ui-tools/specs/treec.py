from specs.lib import *
F = U + '/🌳️retained/🌳️tree/🔀️reconcile/🦀️.rs'
CH = [
    ch('📄', 'page-reconcile-and-cursor', 'page_reconcile_and_cursor', [(40, 235)],
       'Page reconcile and cursor: the document tree page reconcile, the reconcile depth, faults and steps, the retired scene and witness, the reconcile phase and the `UiDocumentReconcileCursor`.'),
    ch('🧾', 'record-routing-and-actions', 'record_routing_and_actions', [(237, 507)],
       'Record routing and actions: the subtree consumption, the space, edge and stack metrics, the layout routing flags, the labels, icons and menus, the presence, action and intent bindings and the scene lane merge.'),
    ch('🖼', 'surface-scene-and-row-nodes', 'surface_scene_and_row_nodes', [(509, 690)],
       'Surface scene and row nodes: the surface scene node, the drag data, the data attributes, the row actions, the tree window and the table section and row nodes.'),
    ch('🌲', 'tree-item-nodes', 'tree_item_nodes', [(692, 913)],
       'Tree item nodes: the tree item node, its tone, detail, inline toolbar and toolbar, the row content lines and the item controls.'),
    ch('🎛', 'widget-nodes', 'widget_nodes', [(915, 1074)],
       'Widget nodes: the input commit action and the input, select, toggle, button, key value, slider, number stepper, ring, icon select and progress nodes.'),
    ch('🎞', 'media-transport-and-node-from-record', 'media_transport_and_node_from_record', [(1076, 1313)],
       'Media transport and node from record: the media transport extension validation and host status and the `ui_node_from_record` conversion.'),
    ch('🌳', 'ui-tree-reconcile', 'ui_tree_reconcile', [(1315, 1595)],
       'Ui tree reconcile: the `UiTree` document reconcile that mounts, updates and retires nodes from a record table.'),
    ch('🔽', 'composite-expansion-rows', 'composite_expansion_rows', [(1681, 1826)],
       'Composite expansion rows: the select item rows, the item value arguments, the tree section and item rows and the own field and layout change checks.'),
    ch('🧰', 'reconcile-testkit-helpers', 'reconcile_testkit_helpers', [(1598, 1678), (1828, 1934)],
       'Reconcile testkit helpers: the variant discriminant, explicit ids, node keys and children of a record and the testkit `UiTree` accessors.'),
]
SPEC = dict(orig='treec.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='treec.rs'))
