#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🌲️concrete-forest/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️ExampleLaws
use crate::standards::v1::subsets::any::schema::inferences::Process3dInference;
use crate::Process3dSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn example_parses_to_the_reference_stock() {
    let snapshot = <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).expect("concrete forest example parses");
    assert_eq!(snapshot.stock_payload.solid, crate::WorkingSolid::Reference { reference_id: crate::REFERENCE_SOLID_CONCRETE_FOREST_LEFT.into() });
    assert_eq!(snapshot.step_payloads.len(), 7);
    assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
}
//#endregion 🧪️ExampleLaws
