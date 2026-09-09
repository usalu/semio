use super::*;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use store::ArtifactDsl;

#[test]
fn layout_pdf_page_collection_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️pdf-page-text.json")).expect("neutral PDF fixture");
    let snapshot: LayoutSnapshot = dsl::os_pack::from_json_str(&fixture["snapshot"].to_string()).expect("owned layout fixture");
    let exported = crate::io::export::serializers::artifacts::pdf::v1_4::base::serialize(&snapshot).expect("PDF export");
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(&exported)).expect("independent PDF JSON oracle");
    assert_eq!(actual["pages"].as_array().expect("pages").len(), fixture["exportPages"].as_u64().expect("count") as usize);
    assert_eq!(actual["pages"][0]["width"], fixture["width"]);
    assert_eq!(actual["pages"][0]["height"], fixture["height"]);
    assert_eq!(actual["pages"][0]["text"], snapshot.print_dsl());
    assert_eq!(crate::io::import::deserializers::artifacts::pdf::v1_4::base::deserialize(&exported).expect("single-page import"), snapshot);
    let text = snapshot.print_dsl();
    let (first, second) = text.split_once('\n').expect("multiline document");
    let split = PdfSnapshot { schema: exported.schema, pages: [first, second].into_iter().map(|text| PageDoc { width: 612.0, height: 792.0, text: text.into() }).collect() };
    assert_eq!(crate::io::import::deserializers::artifacts::pdf::v1_4::base::deserialize(&split).expect("all-page import"), snapshot);
}
