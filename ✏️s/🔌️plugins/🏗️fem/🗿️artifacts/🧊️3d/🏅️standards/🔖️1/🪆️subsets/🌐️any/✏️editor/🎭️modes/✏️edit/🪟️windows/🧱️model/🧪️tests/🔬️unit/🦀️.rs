use super::*;
use crate::editor::fem3d::unit_tests::context::{fem3d_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_fem3d_model_scene() {
    let mut app = fem3d_app();
    let json = render_body(&mut app, FEM3D_BODY_MODEL);
    assert!(json.contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
    let mut app = fem3d_app();
    crate::editor::fem3d::unit_tests::context::dispatch(&mut app, crate::editor::fem3d::Fem3dCommand::SetActiveExample(crate::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &crate::viewport::INITIAL).expect("fixture surface admission");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world scene");
    assert!(scene.meshes_json.contains("solid-sol1"), "expected a solid mesh for the example fixture: {}", scene.meshes_json);
    assert!(scene.instances_json.contains("\"id\":\"e1\""), "expected a single oriented box instance per member keyed by the member id: {}", scene.instances_json);
    assert!(scene.instances_json.contains("\"interactionGranularityId\":\"element\""), "every member instance names its granularity: {}", scene.instances_json);
    assert!(scene.instances_json.contains("\"id\":\"s_00\""), "expected a support cone keyed by the support id: {}", scene.instances_json);
    assert!(scene.instances_json.contains("\"interactionId\":\"l2\""), "expected the nodal load arrow glyphs to redirect onto their load: {}", scene.instances_json);
    assert!(!scene.instances_json.contains("\"sphere\""), "sphere markers should be gone: {}", scene.instances_json);
    assert_eq!(scene.domain_id.as_deref(), Some(crate::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN), "the model window binds the fem3d interaction domain");
}

/// 🧭️ With the transform utility armed and a node selected, the selection record arms the host
/// gumball at the node with live dispatch; without a selection it stays inactive.
#[test]
fn transform_utility_arms_the_gumball_over_the_selection() {
    let doc = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot();
    let window = config::Fem3dModelWindowConfig::default();
    let armed = model_scene(&doc, &window, &crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::selecting(["n20_l1"]), true);
    let record: dsl::json::Value = dsl::json::parse(&armed.selection_json).expect("selection json");
    assert_eq!(record.get("gumballActive").and_then(dsl::json::Value::as_bool), Some(true));
    assert_eq!(record.get("gumballLiveDispatch").and_then(dsl::json::Value::as_bool), Some(true));
    assert_eq!(record.get("transformMode").and_then(dsl::json::Value::as_str), Some("transform"));
    let pivot: Vec<f64> = record.get("gumballTarget").and_then(dsl::json::Value::as_array).expect("pivot").iter().filter_map(dsl::json::Value::as_f64).collect();
    assert_eq!(pivot, vec![8.0, 0.0, 2.8]);
    let idle = model_scene(&doc, &window, &crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::default(), true);
    let record: dsl::json::Value = dsl::json::parse(&idle.selection_json).expect("selection json");
    assert_eq!(record.get("gumballActive").and_then(dsl::json::Value::as_bool), Some(false));
    let plain = model_scene(&doc, &window, &crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::selecting(["n20_l1"]), false);
    let record: dsl::json::Value = dsl::json::parse(&plain.selection_json).expect("selection json");
    assert!(record.get("transformMode").is_none(), "no gumball descriptor without the transform utility");
    assert_eq!(record.get("ids").and_then(dsl::json::Value::as_array).map(Vec::len), Some(1));
}
