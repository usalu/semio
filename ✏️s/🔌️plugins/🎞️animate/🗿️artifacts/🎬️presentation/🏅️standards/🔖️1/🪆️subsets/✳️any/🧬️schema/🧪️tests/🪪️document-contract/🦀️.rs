//! 🧪️ Presentation native codecs preserve the shared durable child identity contract.

use crate::{PresentationDiff, PresentationSnapshot};
use crate::standards::v1::subsets::any::schema::PresentationArtifact;
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn presentation_document_contract_round_trips_children_and_rejects_foreign_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let default = PresentationSnapshot::default();
    assert_eq!(default.presentation.child_id, default.presentation.target.artifact_id);
    assert_eq!(default.animation.child_id, default.animation.target.artifact_id);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&default).unwrap().len(), 2);
    let source = fixture["document"].to_string();
    let artifact: PresentationArtifact = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&artifact)).unwrap();
    assert_eq!(observed, fixture["document"]);
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
    assert_eq!(PresentationSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(PresentationSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let delta: PresentationDiff = semio_framework_pack_json::from_json_str(&fixture["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&delta)).unwrap();
    assert_eq!(observed, fixture["diff"]);
    for row in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<PresentationArtifact>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
        assert!(semio_framework_pack_json::from_json_str::<PresentationSnapshot>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
    for row in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<PresentationDiff>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
}
