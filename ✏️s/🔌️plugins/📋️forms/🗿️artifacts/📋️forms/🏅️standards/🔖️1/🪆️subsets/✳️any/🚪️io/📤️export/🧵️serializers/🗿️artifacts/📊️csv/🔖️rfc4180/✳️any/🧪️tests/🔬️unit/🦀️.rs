use super::*;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use crate::schema::onboarding_example_spec;

#[semio_framework_async_macros::async_test]
async fn export_flattens_one_row_per_question_plus_header() {
    let spec = onboarding_example_spec();
    let question_count: usize = forms_steps(&spec).iter().map(|step| step.blocks.len()).sum();
    let IoOutcome { value: IoPayload::Binary(bytes), .. } = FormsIntoCsv::serialize(&spec).await.expect("serialize") else {
        panic!("expected a binary payload");
    };
    let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(csv.records.len(), question_count + 1, "header + one row per question, no side channel");
    assert_eq!(csv.records[0].fields[0].value, "id");
}

/// 🔮️ The third-party `csv` and `zip` readers (test-only) read the real bytes stdio's codecs write.
#[semio_framework_async_macros::async_test]
async fn csv_and_zip_are_real_files_read_by_third_party_readers() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use std::io::Read;
    let spec = onboarding_example_spec();
    let IoOutcome { value: IoPayload::Binary(pack), .. } = FormsIntoCsv::serialize(&spec).await.expect("csv") else { panic!("binary") };
    let text = semio_s_artifact_stdio_csv::schema::snapshot::encode_csv(&<CsvSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("csv pack"));
    let mut reader = csv::Reader::from_reader(text.as_bytes());
    assert_eq!(reader.headers().expect("header").iter().collect::<Vec<_>>(), vec!["id", "stepId", "label", "kind", "required"]);
    assert_eq!(reader.records().count(), forms_steps(&spec).iter().map(|step| step.blocks.len()).sum::<usize>());
    let zip = export::zip::v2_0::any::FormsIntoZip::serialize(&spec).await.expect("zip");
    let IoPayload::Binary(zip_pack) = &zip.value else { panic!("binary") };
    let bytes = semio_s_artifact_stdio_zip::io::encode_zip(&<semio_s_artifact_stdio_zip::ZipSnapshot as store::ArtifactPack>::decode_pack(zip_pack).expect("zip pack")).expect("zip bytes");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("the zip crate opens the archive");
    let mut member = String::new();
    archive.by_name(&semio_s_artifact_stdio_zip::io::document_archive_member::<FormsSnapshot>()).expect("dsl member").read_to_string(&mut member).expect("utf-8");
    assert_eq!(member, store::ArtifactDsl::print_dsl(&spec));
    assert_eq!(import::zip::v2_0::any::ZipIntoForms::deserialize(&zip.value).await.expect("zip back").value, spec);
}
