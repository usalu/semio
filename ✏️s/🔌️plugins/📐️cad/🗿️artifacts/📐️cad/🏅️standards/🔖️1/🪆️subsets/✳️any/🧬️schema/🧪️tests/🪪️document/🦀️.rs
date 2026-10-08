//! 🧪️ CAD native codecs preserve exact model and drawing child identities.

use crate::schema::CadArtifact;
use crate::{CadDiff, CadSnapshot};
use store::{ArtifactDsl, ArtifactPack};
use semio_framework_os_kernel::{DiffBinary, DiffText};

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
    let references = &snapshot.references_by_model_definition_id;
    let expected = fixture["document"]["referencesByModelDefinitionId"].as_object().unwrap();
    assert!(matches!(semio_framework_dsl_record::DslField::projection_view(references, &[]).unwrap(), semio_framework_dsl_record::native_encoding::FieldProjectionView::Map(count) if count == expected.len()));
    for (index, (key, rows)) in expected.iter().enumerate() {
        assert_eq!(semio_framework_dsl_record::DslField::projection_key(references, &[], index).unwrap(), key);
        assert!(matches!(semio_framework_dsl_record::DslField::projection_view(references, &[index]).unwrap(), semio_framework_dsl_record::native_encoding::FieldProjectionView::List(count) if count == rows.as_array().unwrap().len()));
    }
    let diff: CadDiff = semio_framework_pack_json::from_json_str(&fixture["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&diff)).unwrap();
    assert_eq!(observed, fixture["diff"]);
    assert_eq!(CadDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
    assert_eq!(CadDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
    let replacement = CadDiff { artifact: Some(Box::new(artifact.clone())), ..Default::default() };
    assert_eq!(CadDiff::parse_diff(&replacement.print_diff()).unwrap(), replacement);
    assert_eq!(CadDiff::decode_diff(&replacement.encode_diff().unwrap()).unwrap(), replacement);

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
    let alias=crate::cad_model_child("model-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "model-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } }).unwrap();
    assert_eq!(alias.child_id,"model-a");
    assert_eq!(alias.target.artifact_id,"model-b");
    assert!(crate::cad_model_child("model-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "model-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() } }).is_err());
    assert!(crate::cad_drawing_child("drawing-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "drawing-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() } }).is_ok());
    eprintln!("[DEBUG] Cad document preserves two exact child identities, {} reference keys, snapshot text/binary and sparse/replacement diff round trips against the serde_json corpus", expected.len());
}

#[test]
fn cad_brep_events_preserve_ordered_geometry_ownership_and_exact_inverse() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let cases = [
        (
            include_str!("../../../🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🗣️.dsl.semio"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🔺️diff/🔣️.json"),
        ),
        (
            include_str!("../../../🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🗣️.dsl.semio"),
            include_str!("../../../🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🔺️diff/🔣️.json"),
        ),
    ];
    for (before, after, input, text, diff) in cases {
        let before: CadSnapshot = semio_framework_pack_json::from_json_str(before, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let after: CadSnapshot = semio_framework_pack_json::from_json_str(after, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: crate::CadMutation = semio_framework_pack_json::from_json_str(input, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).unwrap(), serde_json::from_str::<serde_json::Value>(input).unwrap());
        assert_eq!(crate::CadMutation::parse_op(text.trim()).unwrap(), mutation);
        assert_eq!(crate::CadMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        let outcome = mutation.diff(&before);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).unwrap(), serde_json::from_str::<serde_json::Value>(diff).unwrap());
        assert_eq!(outcome.diff().apply(&before).unwrap(), after);
        let mut restored = after;
        for inverse in mutation.inverse(&before).unwrap() { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }
        assert_eq!(restored, before);
        eprintln!("[DEBUG] Cad topology event preserves exact middle-position inverse and native text/binary against the independent serde_json corpus");
    }
}
