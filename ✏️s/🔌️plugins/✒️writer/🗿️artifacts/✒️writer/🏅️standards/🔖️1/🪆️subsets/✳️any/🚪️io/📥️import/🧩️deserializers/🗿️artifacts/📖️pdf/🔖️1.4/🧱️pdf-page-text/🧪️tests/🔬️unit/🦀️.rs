use semio_framework::io_schema::IoPayload;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
#[allow(unused_imports)]
use crate::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf::v1_4::base::*;
#[semio_framework_async_macros::async_test]
async fn pdf_page_text_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️pdf-page-text/🧪️tests/🧫️fixtures/🔣️page-text.json")).unwrap();
    for vector in vectors["cases"].as_array().unwrap() {
        let value = serde_json::json!({"schema":"stdio.pdf","pages":vector["pages"]});
        let pdf: PdfSnapshot = semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let bytes = <PdfSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&pdf);
        let outcome = PdfIntoWriter::deserialize(&IoPayload::Binary(bytes)).await.expect("deserialize");
        assert_eq!(crate::writer_text(&outcome.value), vector["expected"].as_str().unwrap(), "{}", vector["name"]);
        assert_eq!(pdf.pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>().join("\n"), vector["expected"].as_str().unwrap());
    }
}
