#[semio_framework_async_macros::async_test]
async fn primary_asset_fails_and_every_failure_has_a_remedy() {
    let text = include_str!("../../../../🖼️assets/💥failing-prestressed-beam/💥failing-prestressed-beam/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <semio_s_artifact_norm_en1992_model::En1992Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("parse prestressed example");
    assert!(snap.members.iter().any(|m| m.prestress.is_some()));
    let report = semio_s_artifact_norm_en1992_inferences::evaluate(&snap);
    let fails: Vec<_> = report.failing().collect();
    assert!(!fails.is_empty(), "the failing prestressed beam emits no failing check");
    for fail in &fails {
        assert!(!fail.remedies.is_empty(), "fail {} must carry a remedy", fail.id);
    }
}
