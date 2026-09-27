#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🧵compliant-prestressed-beam/🧵compliant-prestressed-beam/🗣️.dsl.semio");
    assert!(text.len() > 8);
    let snap = <crate::En1992Snapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse prestressed example");
    assert!(snap.members.iter().any(|m| m.prestress.is_some()));
    assert!(crate::standards::v1::subsets::any::schema::inferences::evaluate(&snap).complies());
}
