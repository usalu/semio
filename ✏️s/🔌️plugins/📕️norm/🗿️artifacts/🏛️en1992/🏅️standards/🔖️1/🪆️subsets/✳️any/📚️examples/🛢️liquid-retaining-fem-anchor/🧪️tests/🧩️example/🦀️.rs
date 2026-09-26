#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <crate::En1992Snapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse liquid example");
    assert!(!snap.members.is_empty());
    assert!(snap.members.iter().any(|m| m.tightness.is_some() || m.kind == crate::MemberKind::LiquidRetaining));
    let report = crate::standards::v1::subsets::any::schema::inferences::evaluate(&snap);
    assert!(!report.checks.is_empty(), "liquid example must produce checks");
    assert!(
        report.checks.iter().any(|c| c.id.contains("liquid") || c.id.contains("7.3") || c.explanation.en.to_lowercase().contains("crack") || c.explanation.de.to_lowercase().contains("riss")),
        "missing EN 1992-3 crack-class checks: {:?}",
        report.checks.iter().map(|c| c.id.clone()).collect::<Vec<_>>()
    );
}


#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::En1992Inference;
    use crate::En1992Snapshot;
    use protocol::Inference;
    let snapshot = En1992Snapshot::default();
    assert_eq!(En1992Inference::infer(&snapshot), En1992Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::En1992Inference;
    use crate::En1992Snapshot;
    use protocol::Inference;
    assert_eq!(En1992Inference::infer(&En1992Snapshot::default()), En1992Inference::default());
}
