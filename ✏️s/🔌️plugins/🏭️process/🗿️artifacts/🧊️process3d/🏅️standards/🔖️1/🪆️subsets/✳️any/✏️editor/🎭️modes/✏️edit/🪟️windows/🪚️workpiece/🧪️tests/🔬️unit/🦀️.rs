use super::*;
use crate::editor::process3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    let doc = Process3dSnapshot::default();
    let engagement = engagement(&doc, &Process3dConfig::default(), crate::editor::process3d::config::PROCESS3D_DEFAULT_UTILITY, &crate::editor::process3d::terminology::Process3dLabels::NATIVE_EN);
    assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options");
}

#[semio_framework_async_macros::async_test]
async fn render_world_scene_contains_processed_mesh() {
    let mut app = context::app();
    let node = context::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
}

/// 🪚️ The default timber document (3.0 × 0.2 × 0.3 stock, crosscut + lap joint + dowel) must reach the
/// window as the replayed kernel mesh — `PROCESS3D_FALLBACK_MESH_KIND` (a unit box) is only for a
/// document that carries no stock at all, so a unit-box extent here means the replay silently failed.
#[semio_framework_async_macros::async_test]
async fn render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box() {
    let snapshot = crate::schema::default_document();
    let scene = crate::process_working_scene_from_snapshot(&snapshot);
    assert!(matches!(scene.stock.solid, crate::WorkingSolid::Box { width, .. } if (width - 3.0).abs() < 1e-9), "timber fixture stock: {:?}", scene.stock.solid);
    let mesh = processed_mesh(&scene, snapshot.resolved_up_to).expect("timber replay tessellates");
    let extent = |axis: usize| {
        let values = mesh.positions.iter().skip(axis).step_by(3).map(|value| f64::from(*value));
        values.clone().fold(f64::NEG_INFINITY, f64::max) - values.fold(f64::INFINITY, f64::min)
    };
    assert!((extent(0) - 3.0).abs() < 1e-3, "x extent {} is not the 3.0 beam length", extent(0));
    assert!(extent(1) < 1.0 && extent(2) < 1.0, "y/z extents {}/{} are not the 0.2/0.3 beam section", extent(1), extent(2));
    let (meshes_json, _) = evaluated_preview_payload(&snapshot, &scene);
    assert!(!meshes_json.contains("[-0.5,-0.5,0.5,0.5,-0.5,0.5"), "window payload is the unit-box fallback");
}
