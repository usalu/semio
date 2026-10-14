#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🧵compliant-prestressed-beam/🧵compliant-prestressed-beam/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <semio_s_artifact_norm_en1992_model::En1992Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("parse prestressed example");
    assert!(snap.members.iter().any(|m| m.prestress.is_some()));
    assert!(semio_s_artifact_norm_en1992_inferences::evaluate(&snap).complies());
}
