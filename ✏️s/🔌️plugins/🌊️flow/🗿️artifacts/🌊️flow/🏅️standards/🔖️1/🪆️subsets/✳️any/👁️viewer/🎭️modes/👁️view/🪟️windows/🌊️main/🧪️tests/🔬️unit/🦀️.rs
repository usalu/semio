use crate::FlowSnapshot;
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use crate::schema::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
#[allow(unused_imports)]
use flow::{flow_backed_node_graph_extras, FlowEvalSession, FLOW_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_os_infinite_directed_presentation::DagHostSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::scene_surface;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphScene;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_os_kernel::Viewport2d;
#[allow(unused_imports)]
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)] use semio_framework_ui_declaration::ui::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use crate::viewer::flow::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene_for_the_default_document() {
    let document = FlowSnapshot::default();
    let node = render(&document).expect("render");
    let tree = semio_framework_plugin::app::built_to_component_tree(node);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection");
    assert!(json.contains("node-graph"));
}
