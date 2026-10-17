//! 🧪️ CAD native codecs preserve exact model and drawing child identities.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_cad_cad_model::CadArtifact;
use semio_s_artifact_cad_cad_model_diff::CadDiff;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
use semio_framework_os_kernel::{DiffBinary, DiffText};

#[test]
fn cad_document_contract_round_trips_exact_child_identities() {
    let fixture: serde_json::Value = serde_json::from_str(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json").leak()).unwrap();
    let source = fixture["document"].to_string();
    let artifact: CadArtifact = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot: CadSnapshot = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(CadArtifact::from_snapshot(snapshot.clone()), artifact);
    assert_eq!(semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
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
    let alias=semio_s_artifact_cad_cad_model::snapshot::cad_model_child("model-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "model-b".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } }).unwrap();
    assert_eq!(alias.child_id,"model-a");
    assert_eq!(alias.target.artifact_id,"model-b");
    assert!(semio_s_artifact_cad_cad_model::snapshot::cad_model_child("model-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "model-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() } }).is_err());
    assert!(semio_s_artifact_cad_cad_model::snapshot::cad_drawing_child("drawing-a", &semio_framework_artifact_reference::ArtifactRef { artifact_id: "drawing-a".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() } }).is_ok());
}

#[test]
fn cad_brep_events_preserve_ordered_geometry_ownership_and_exact_inverse() {
    use protocol::{Mutation, OpBinary, OpText};
    let cases = [
        (
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/➡️after/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/➡️after/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🗣️.dsl.semio")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🗣️.dsl.semio").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🔺️diff/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🔺️diff/🔣️.json").leak(),
        ),
        (
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/➡️after/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/➡️after/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🗣️.dsl.semio")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🗣️.dsl.semio").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🔺️diff/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🔺️diff/🔣️.json").leak(),
        ),
    ];
    for (before, after, input, text, diff) in cases {
        let before: CadSnapshot = semio_framework_pack_json::from_json_str(before, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let after: CadSnapshot = semio_framework_pack_json::from_json_str(after, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: semio_s_artifact_cad_cad_mutations::CadMutation = semio_framework_pack_json::from_json_str(input, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).unwrap(), serde_json::from_str::<serde_json::Value>(input).unwrap());
        assert_eq!(semio_s_artifact_cad_cad_mutations::CadMutation::parse_op(text.trim()).unwrap(), mutation);
        assert_eq!(semio_s_artifact_cad_cad_mutations::CadMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        let outcome = mutation.diff(&before);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).unwrap(), serde_json::from_str::<serde_json::Value>(diff).unwrap());
        assert_eq!(protocol::apply_diff(outcome.diff(), &before).unwrap(), after);
        let mut restored = after;
        for inverse in mutation.inverse(&before).unwrap() { restored = protocol::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, before);
    }
}

/// ⚖️ The topology events' concrete inverses sum to exactly the negative of their forward diffs, middle position included.
#[semio_framework_async_macros::async_test]
async fn cad_brep_events_invert_to_the_negative_diff() {
    let cases = [
        (
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json").leak(),
        ),
        (
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json").leak(),
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json")).expect("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json").leak(),
        ),
    ];
    for (before, input) in cases {
        let before: CadSnapshot = semio_framework_pack_json::from_json_str(before, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: semio_s_artifact_cad_cad_mutations::CadMutation = semio_framework_pack_json::from_json_str(input, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
    }
}
