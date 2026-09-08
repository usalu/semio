
use super::*;
use crate::{ShootingCamera, ShootingSavedCamera, ShootingShot};
use protocol::Inference;

//#region 🧸️Fixtures
fn sample_snapshot() -> ShootingSnapshot {
    ShootingSnapshot {
        saved_cameras: vec![ShootingSavedCamera { id: "cam-1".into(), label: "Front".into(), camera: ShootingCamera::default() }],
        shots: vec![ShootingShot { id: "shot-1".into(), label: "Shot 1".into(), width: 1024, height: 768, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: Some("cam-1".into()) }],
        ..ShootingSnapshot::default()
    }
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(ShootingInference::infer(&snapshot), ShootingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(ShootingInference::infer(&ShootingSnapshot::default()), ShootingInference::default());
}
//#endregion 🧪️InferenceLaws
