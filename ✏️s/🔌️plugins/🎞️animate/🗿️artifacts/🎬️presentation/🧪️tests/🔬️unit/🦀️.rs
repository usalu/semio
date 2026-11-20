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


/// 🏗️ The UTF-8 DSL carrier preserves the complete presentation snapshot.
#[test]
fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::PresentationIntoTxt;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::TxtIntoPresentation;
    use semio_framework::io_schema::IoPayload;
    use semio_framework_async::poll::resolve_ready;
    use semio_framework_test_original_snapshot_owners::{original_grant, with_decoding, with_encoding};
    use semio_os_kernel::io::io_mechanism::{ArchiveChildren, Deserializer, IoRunControl, Serializer};

    let snapshot = default_presentation_snapshot();
    let exported = with_encoding(|owner| {
        let mut control = IoRunControl::encoding(owner.native(), original_grant());
        resolve_ready(PresentationIntoTxt::serialize(&snapshot, &ArchiveChildren::empty(), &mut control))
    }).expect("DSL text export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = with_decoding(|owner| {
        let mut control = IoRunControl::decoding(owner.native(), original_grant());
        resolve_ready(TxtIntoPresentation::deserialize(&IoPayload::Text(text), &mut control))
    }).expect("DSL text import");
    assert_eq!(back.value, snapshot);
}
