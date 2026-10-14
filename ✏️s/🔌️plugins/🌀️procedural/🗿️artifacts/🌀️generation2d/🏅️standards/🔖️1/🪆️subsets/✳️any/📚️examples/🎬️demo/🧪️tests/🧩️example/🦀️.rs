#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    use semio_s_artifact_procedural_2d_model::inferences::Generation2dInference;
    use protocol::Inference;

    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let snapshot = semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead::new(semio_s_artifact_procedural_2d_io_text::snapshot::parse_dsl(text).expect("example dsl parses"));
    assert_eq!(Generation2dInference::infer(&snapshot).expect("valid materialized inference fixture"), Generation2dInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[test]
fn inference_default_law() {
    use semio_s_artifact_procedural_2d_model::inferences::Generation2dInference;
    use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
    use protocol::Inference;

    assert_eq!(Generation2dInference::infer(&Generation2dSnapshot::default()).expect("valid materialized inference fixture"), Generation2dInference::default());
}
//#endregion 🧪️InferenceLaws
