from specs.lib import *
F = U + '/📜️declaration/🦀️.rs'
INC1 = 'include!("../🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs");'
INC2 = 'include!("../🧪️tests/🔬️targets-wgpu-component-ui-value-round-trip/🦀️.rs");'
UIC = [
    ch('📚', 'layout-and-text-nodes', 'layout_and_text_nodes', [(18, 129)],
       'Layout and text nodes: the stack node, the drop overlay and the text, button, separator and image nodes.'),
    ch('⌨', 'input-select-toggle-and-group-nodes', 'input_select_toggle_and_group_nodes', [(131, 295)],
       'Input, select, toggle and group nodes: the input, select item and select, toggle, group and key value nodes.'),
    ch('🎚', 'slider-stepper-and-control-nodes', 'slider_stepper_and_control_nodes', [(297, 518)],
       'Slider, stepper and control nodes: the slider, number stepper, ring, icon select and progress nodes and the `UiControlNode` union.'),
    ch('🌲', 'field-section-and-tree-nodes', 'field_section_and_tree_nodes', [(520, 841)],
       'Field, section and tree nodes: the field and section nodes, the tree action, window, item, section and tree nodes and the tree presence stamping.'),
    ch('🔍', 'inspector-helpers', 'inspector_helpers', [(843, 1241)],
       'Inspector helpers: the inspector field groups, the mixed value helpers, the read-only, stepper, toggle and vec3 fields and the declarative section to tree conversion.'),
    ch('🏔', 'surface-kind-and-world-records', 'surface_kind_and_world_records', [(1245, 1469)],
       'Surface kind and world records: the `SurfaceKind` and the world mesh lod, lod, chunking and camera records with their json builders.'),
    ch('🍱', 'table-cells-and-component-scene-nodes', 'table_cells_and_component_scene_nodes', [(1484, 1608)],
       'Table cells and component scene nodes: the table cells, the block palette entries, the external slot node and the component scene node.'),
    ch('🧬', 'ui-node-enum', 'ui_node_enum', [(1612, 1744)],
       'Ui node enum: the `UiNode` union and its constructors, accessors and conversions.'),
    ch('🎬', 'scene-actions-and-node-conversions', 'scene_actions_and_node_conversions', [(1746, 1872)],
       'Scene actions and node conversions: the text editor views, the node graph, text editor, board, ink canvas and tiled map action modules and the node and control conversions.'),
    ch('🏗', 'scene-builders', 'scene_builders', [(1874, 2024)],
       'Scene builders: the component scene constructor, the scene builders for each scene kind and the text identifier occurrences.'),
    ch('🧯', 'status-builders', 'status_builders', [(2027, 2155)],
       'Status builders: the empty state, error state, recovery panel and import drop zone builders.'),
]
SPEC = dict(
    orig='decl.rs', file=F, children=[], expect_missing=['pub mod ui {', '}'],
    external=mounted(F, orig='decl.rs'),
    patch={'🧩️ui-graph': [(INC1, INC1.replace('"../', '"../../')), (INC2, INC2.replace('"../', '"../../'))]},
    inner=dict(mod='ui', indent=4, folder='🧩️ui-graph', nest=False, rebase_super=True,
               doc='//! 🧩️ Ui graph module: the node, control, tree, inspector, surface, scene and status declarations with the shared imports of the former inline module. See [`super`].',
               children=UIC),
)
