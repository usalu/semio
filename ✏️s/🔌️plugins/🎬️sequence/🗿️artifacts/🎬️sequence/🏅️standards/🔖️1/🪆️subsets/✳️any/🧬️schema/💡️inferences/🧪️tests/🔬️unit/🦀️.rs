use super::*;
use crate::{SequenceEdge, SequenceFixture, SequenceStep, StepParams};
use protocol::Inference;

//#region 🧸️Fixtures
fn step(id: &str) -> SequenceStep {
    SequenceStep { id: id.into(), kind: "state.set".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }
}

fn sample_snapshot() -> SequenceSnapshot {
    SequenceSnapshot::from_fixture(SequenceFixture { schema: crate::SEQUENCE_DOCUMENT_SCHEMA.into(), steps: vec![step("a"), step("b")], edges: vec![SequenceEdge { id: "e1".into(), from: "a".into(), to: "b".into() }] })
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = neural_engine::ColdOwner::new(sample_snapshot());
    assert_eq!(SequenceInference::infer(&snapshot), SequenceInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SequenceInference::infer(&neural_engine::ColdOwner::new(SequenceSnapshot::default())), SequenceInference::default());
}
//#endregion 🧪️InferenceLaws
