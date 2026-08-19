#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramInference;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Inference;

#[test]
async fn inference_determinism_law() {
    let snapshot = ProgramSnapshot::default();
    assert_eq!(ProgramInference::infer(&snapshot), ProgramInference::infer(&snapshot));
}

#[test]
async fn inference_default_law() {
    assert_eq!(ProgramInference::infer(&ProgramSnapshot::default()), ProgramInference::default());
}
//#endregion 🧪️InferenceLaws
