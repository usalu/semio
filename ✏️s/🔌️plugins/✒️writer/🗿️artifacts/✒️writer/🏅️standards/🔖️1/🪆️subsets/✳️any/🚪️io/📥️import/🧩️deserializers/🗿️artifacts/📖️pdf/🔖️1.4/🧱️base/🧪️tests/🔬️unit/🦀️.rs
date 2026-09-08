
use super::*;
#[semio_framework_async_macros::async_test]
async fn pdf_page_text_vectors_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️page-text.json")).unwrap();
    for vector in vectors["cases"].as_array().unwrap() {
        let value = serde_json::json!({"schema":"stdio.pdf","pages":vector["pages"]});
        let pdf: PdfSnapshot = dsl::os_pack::json::from_json_str(&value.to_string()).unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&pdf);
        let outcome = PdfIntoWriter::deserialize(&IoPayload::Binary(bytes)).await.expect("deserialize");
        assert_eq!(crate::writer_text(&outcome.value), vector["expected"].as_str().unwrap(), "{}", vector["name"]);
        assert_eq!(pdf.pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>().join("\n"), vector["expected"].as_str().unwrap());
    }
}
