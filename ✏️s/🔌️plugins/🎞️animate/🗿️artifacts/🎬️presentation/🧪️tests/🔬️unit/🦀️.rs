use super::*;

#[test]
fn presentation_snapshot_schema_is_animate_presentation() {
    assert_eq!(default_presentation_snapshot().schema, PRESENTATION_DOCUMENT_SCHEMA);
}

#[test]
fn artifact_kind_matches_the_store_schema() {
    assert_eq!(artifact_kind().schema, PRESENTATION_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().id, PRESENTATION_DOCUMENT_SCHEMA);
}


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::default_presentation_snapshot();
    let exported = export::txt::v_utf_8::any::PresentationIntoTxt::serialize(&snapshot).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoPresentation::deserialize(&IoPayload::Text(text)).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}
