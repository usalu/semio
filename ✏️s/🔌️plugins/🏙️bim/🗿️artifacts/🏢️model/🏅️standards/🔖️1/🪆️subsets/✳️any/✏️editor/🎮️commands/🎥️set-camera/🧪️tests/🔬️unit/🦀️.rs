use super::*;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use semio_framework_ui_locale::Locale;

fn window(kind: &str) -> BimDispatchCtx {
    let view = view(Locale::En, &[("window", kind)], Some("window"));
    BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None)
}

fn pose2d(x: f64) -> SetCamera {
    SetCamera { camera2d: Some(store::Viewport2d { x, y: 1.0, zoom: 20.0 }), camera3d: None }
}

fn pose3d() -> SetCamera {
    SetCamera { camera2d: None, camera3d: Some(store::Viewport3dOrbit { position: [5.0, -5.0, 4.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: None }) }
}

fn set(kind: &str, payload: &SetCamera) -> (Result<Emit<ModelMutation, NoConfigMutation>, Fault>, BimDispatchCtx) {
    let snapshot = demo();
    let mut ctx = window(kind);
    let result = run(&snapshot, |doc, cfg| handle(payload, doc, cfg, &mut ctx));
    (result, ctx)
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

#[semio_framework_async_macros::async_test]
async fn each_window_kind_takes_its_own_pose_and_writes_only_its_config() {
    for (kind, payload) in [(plan::WINDOW_KIND_ID, pose2d(2.0)), (section::WINDOW_KIND_ID, pose2d(3.0)), (world::WINDOW_KIND_ID, pose3d())] {
        let (result, _) = set(kind, &payload);
        let emit = result.unwrap_or_else(|fault| panic!("{kind}: {}", fault.message));
        assert_eq!(emit.window_config_mutations.len(), 1);
        assert!(emit.artifact_mutations.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn a_plan_camera_is_shared_as_presence_and_a_world_camera_is_not() {
    let (_, plan_ctx) = set(plan::WINDOW_KIND_ID, &pose2d(2.0));
    assert_eq!(plan_ctx.presence_out.len(), 1);
    let (_, world_ctx) = set(world::WINDOW_KIND_ID, &pose3d());
    assert!(world_ctx.presence_out.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_pose_for_the_wrong_window_kind_or_no_window_or_a_non_finite_pose_is_refused() {
    assert_eq!(code(set(plan::WINDOW_KIND_ID, &pose3d()).0), Some("bim.camera.pose-mismatch".to_string()));
    assert_eq!(code(set(world::WINDOW_KIND_ID, &pose2d(1.0)).0), Some("bim.camera.pose-mismatch".to_string()));
    assert_eq!(code(set(plan::WINDOW_KIND_ID, &pose2d(f64::NAN)).0), Some("bim.camera.invalid".to_string()));
    let snapshot = demo();
    let mut windowless = BimDispatchCtx::default();
    assert_eq!(code(run(&snapshot, |doc, cfg| handle(&pose2d(1.0), doc, cfg, &mut windowless))), Some("bim.camera.window-required".to_string()));
}
