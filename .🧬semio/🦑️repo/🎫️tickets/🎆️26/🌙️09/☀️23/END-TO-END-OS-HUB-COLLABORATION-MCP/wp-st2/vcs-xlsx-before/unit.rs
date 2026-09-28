use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{csv::v_rfc4180::any::VcsIntoCsv, xlsx::v_ecma_376::any::VcsIntoXlsx, zip::v2_0::any::VcsIntoZip};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::{csv::v_rfc4180::any::CsvIntoVcs, xlsx::v_ecma_376::any::XlsxIntoVcs, zip::v2_0::any::ZipIntoVcs};
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;
use std::io::Read;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample() -> VcsSnapshot {
    VcsSnapshot { schema: "vcs.demo".into(), title: "Release, final".into(), counter: 42, notes: "line one\nline two".into(), status: "open".into(), tags: vec!["a".into(), "b".into()] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn binary(payload: &IoPayload) -> &[u8] {
    let IoPayload::Binary(bytes) = payload else { panic!("binary payload") };
    bytes
}

/// 🔮️ The third-party `csv` reader (test-only) parses the one-record table.
#[semio_framework_async_macros::async_test]
async fn csv_round_trips_and_a_third_party_reader_agrees() {
    let payload = VcsIntoCsv::serialize(&sample()).await.expect("csv").value;
    let text = semio_s_artifact_stdio_csv::schema::snapshot::encode_csv(&<semio_s_artifact_stdio_csv::CsvSnapshot as store::ArtifactPack>::decode_pack(binary(&payload)).expect("pack"));
    let mut reader = csv::Reader::from_reader(text.as_bytes());
    let row = reader.records().next().expect("a value row").expect("row");
    assert_eq!((&row[1], &row[2], &row[3], &row[5]), ("Release, final", "42", "line one\nline two", "a;b"));
    assert_eq!(CsvIntoVcs::deserialize(&payload).await.expect("csv back").value, sample());
}

#[semio_framework_async_macros::async_test]
async fn xlsx_round_trips() {
    let payload = VcsIntoXlsx::serialize(&sample()).await.expect("xlsx").value;
    assert_eq!(XlsxIntoVcs::deserialize(&payload).await.expect("xlsx back").value, sample());
}

/// 🔮️ The third-party `zip` reader (test-only) opens the document archive.
#[semio_framework_async_macros::async_test]
async fn zip_is_a_real_archive_of_the_exact_document() {
    let payload = VcsIntoZip::serialize(&sample()).await.expect("zip").value;
    let bytes = semio_s_artifact_stdio_zip::io::encode_zip(&<semio_s_artifact_stdio_zip::ZipSnapshot as store::ArtifactPack>::decode_pack(binary(&payload)).expect("pack")).expect("zip bytes");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("the zip crate opens the archive");
    let mut member = String::new();
    archive.by_name(&semio_s_artifact_stdio_zip::io::document_archive_member::<VcsSnapshot>()).expect("dsl member").read_to_string(&mut member).expect("utf-8");
    assert_eq!(member, <VcsSnapshot as store::ArtifactDsl>::print_dsl(&sample()));
    assert_eq!(ZipIntoVcs::deserialize(&payload).await.expect("zip back").value, sample());
}
