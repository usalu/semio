//! 🧪️ CAD native codecs preserve exact model and drawing child identities.

use crate::schema::CadArtifact;
use crate::{CadDiff, CadSnapshot};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn cad_document_contract_round_trips_exact_child_identities() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document/🔣️.json")).unwrap();
    let source = fixture["document"].to_string();
    let artifact: CadArtifact = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot: CadSnapshot = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(CadArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
    assert_eq!(CadSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(CadSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let diff: CadDiff = semio_framework_pack_json::from_json_str(&fixture["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&diff)).unwrap();
    assert_eq!(observed, fixture["diff"]);

    for row in fixture["invalidDocuments"].as_array().unwrap().iter().filter(|row| row["kind"] == "field") {
        let mut input = fixture["document"].clone();
        input[row["field"].as_str().unwrap()] = row["value"].clone();
        let text = input.to_string();
        assert!(semio_framework_pack_json::from_json_str::<CadArtifact>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "CadArtifact must reject the document field {}", row["field"]);
        assert!(semio_framework_pack_json::from_json_str::<CadSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "CadSnapshot must reject the document field {}", row["field"]);
    }
    for row in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<CadDiff>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
    let alias=crate::cad_model_child_from_uri("model-a", "model-b!s.stdio.semio@v1/model").unwrap();
    assert_eq!(alias.child_id,"model-a");
    assert_eq!(alias.target.artifact_id,"model-b");
    assert!(crate::cad_model_child_from_uri("model-a", "model-a!s.stdio.semio@v1/drawing").is_err());
    assert!(crate::cad_drawing_child_from_uri("drawing-a", "drawing-a!s.stdio.semio@v1/drawing").is_ok());
}
