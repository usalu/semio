use semio_framework_plugin::NodeGraphScene;
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use crate::{SequenceStep, SequenceWorkingScene};
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphEdgeRecord;
#[allow(unused_imports)]
use semio_framework_plugin::NodeGraphNodeRecord;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_os_kernel::Viewport2d;
use crate::viewer::sequence::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SEQUENCE_VIEW_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_read_only_scene_for_the_default_document() {
    let document = crate::snapshot::schema::default_persisted_snapshot();
    let scene = crate::sequence_derivable_scene(&document).expect("the genesis content is derivable");
    let node = render(&scene).expect("viewer graph");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic graph") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed viewer scene");
    assert_eq!(scene.editable, Some(false));
    assert_eq!(scene.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), ["step-1", "step-2"]);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer graph");
}
