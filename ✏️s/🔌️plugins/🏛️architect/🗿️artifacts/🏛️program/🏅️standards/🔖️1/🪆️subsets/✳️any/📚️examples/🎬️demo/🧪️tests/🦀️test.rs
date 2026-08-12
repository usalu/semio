#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramInference;
use protocol::Inference;

#[test]
fn inference_determinism_law() {
    let snapshot = ProgramSnapshot::default();
    assert_eq!(ProgramInference::infer(&snapshot), ProgramInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(ProgramInference::infer(&ProgramSnapshot::default()), ProgramInference::default());
}
//#endregion 🧪️InferenceLaws
