use super::*;
use crate::{empty_cad_snapshot, sample_scene_fixture::sample_model_child};
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let mut snapshot = empty_cad_snapshot();
    snapshot.shape_model = Some(sample_model_child("inference-law-1"));
    assert_eq!(CadInference::infer(&snapshot), CadInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(CadInference::infer(&empty_cad_snapshot()), CadInference::default());
}
//#endregion 🧪️InferenceLaws
