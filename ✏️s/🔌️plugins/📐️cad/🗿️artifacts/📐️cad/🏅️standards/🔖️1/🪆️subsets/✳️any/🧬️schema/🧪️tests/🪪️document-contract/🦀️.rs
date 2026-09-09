//! 🧪️ CAD native codecs preserve exact model and drawing child identities.

use crate::schema::CadArtifact;
use crate::{CadDiff, CadSnapshot};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn cad_document_contract_round_trips_exact_child_identities() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let source = fixture["document"].to_string();
    let artifact: CadArtifact = dsl::json::from_json_str(&source).unwrap();
    let snapshot: CadSnapshot = dsl::json::from_json_str(&source).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(CadArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
    assert_eq!(CadSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(CadSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let diff: CadDiff = dsl::json::from_json_str(&fixture["diff"].to_string()).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&diff)).unwrap();
    assert_eq!(observed, fixture["diff"]);

    for row in fixture["invalidDocuments"].as_array().unwrap().iter().filter(|row| row["kind"] == "field") {
        let mut input = fixture["document"].clone();
        input[row["field"].as_str().unwrap()] = row["value"].clone();
        let text = input.to_string();
        assert!(dsl::json::from_json_str::<CadArtifact>(&text).is_err());
        assert!(dsl::json::from_json_str::<CadSnapshot>(&text).is_err());
    }
    for row in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<CadDiff>(&row.to_string()).is_err());
    }
    assert!(crate::cad_model_child_from_uri("model-a", "model-b!s.stdio.semio@v1/model").is_err());
    assert!(crate::cad_model_child_from_uri("model-a", "model-a!s.stdio.semio@v1/drawing").is_err());
    assert!(crate::cad_drawing_child_from_uri("drawing-a", "drawing-a!s.stdio.semio@v1/drawing").is_ok());
    eprintln!("[DEBUG] CAD JSON/text/Pack preserve exact model/drawing children and reject document-owned pane selection");
}
