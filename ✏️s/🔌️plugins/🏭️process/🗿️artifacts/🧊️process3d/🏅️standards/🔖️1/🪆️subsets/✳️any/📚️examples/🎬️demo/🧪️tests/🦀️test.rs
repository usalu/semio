#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::artifacts::process3d::Process3dSnapshot;
use crate::artifacts::process3d::standards::v1::subsets::any::schema::inferences::Process3dInference;
use protocol::Inference;

#[test]
async fn inference_determinism_law() {
    let snapshot = Process3dSnapshot::default();
    assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()), Process3dInference::default());
}
//#endregion 🧪️InferenceLaws
