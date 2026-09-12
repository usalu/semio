//! 🧪️ Forms neutral document and sparse-edit laws against independent JSON values.
use crate::{FormsSnapshot, FormsDiff};
use protocol::MutationDiff;
use store::{ArtifactDsl, ArtifactPack};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).expect("neutral vectors")
}

#[test]
fn forms_document_contract_exact_json_and_sparse_edits() {
    let cases = vectors();
    let snapshot: FormsSnapshot = dsl::json::from_json_str(&cases["document"].to_string()).expect("exact document");
    let artifact: crate::schema::FormsArtifact = dsl::json::from_json_str(&cases["document"].to_string()).expect("exact artifact");
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&snapshot)).unwrap(), cases["document"]);
    for value in cases["invalidIdentityDocuments"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<FormsSnapshot>(&value.to_string()).is_err(), "{value}");
        assert!(dsl::json::from_json_str::<crate::schema::FormsArtifact>(&value.to_string()).is_err(), "{value}");
    }
    for value in cases["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<FormsDiff>(&value.to_string()).is_err(), "{value}");
    }
    for case in cases["patchCases"].as_array().unwrap() {
        let before: FormsSnapshot = dsl::json::from_json_str(&case["before"].to_string()).unwrap();
        let diff: FormsDiff = dsl::json::from_json_str(&case["diff"].to_string()).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&diff)).unwrap(), case["diff"]);
        let after = diff.apply(&before).expect("valid sparse change");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&after)).unwrap(), case["after"], "{}", case["name"]);
    }
    println!("[DEBUG] Forms native JSON and sparse deltas match neutral independent JSON values");
}

#[test]
fn forms_document_contract_native_codec_identity() {
    let snapshot = FormsSnapshot::default();
    snapshot.validate().expect("canonical initial children");
    let text = snapshot.print_dsl();
    let decoded = FormsSnapshot::parse_dsl(&text).expect("text decode");
    assert_eq!(decoded, snapshot);
    let bytes = snapshot.encode_pack();
    assert_eq!(FormsSnapshot::decode_pack(&bytes).expect("Pack decode"), snapshot);
    println!("[DEBUG] Forms native text and Pack retain exact canonical child identities");
}

#[test]
fn forms_document_contract_distinct_child_owners_and_typed_refusal() {
    let before = FormsSnapshot::default();
    assert_ne!(before.structure.child_id, before.results.child_id);
    let mut child = before.structure.clone();
    child.child_id = "foreign-child".into();
    let diff = FormsDiff { structure: Some(child), ..Default::default() };
    assert!(diff.apply(&before).is_err());
    assert_eq!(before, FormsSnapshot::default());
    println!("[DEBUG] Forms typed invalid child replacement leaves parent unchanged");
}

#[test]
fn forms_document_contract_rejects_foreign_text_envelope() {
    let text = FormsSnapshot::default().print_dsl();
    let (_, body) = text.split_once('\n').expect("text preamble");
    for header in vectors()["invalidTextEnvelopes"].as_array().unwrap() {
        assert!(FormsSnapshot::parse_dsl(&format!("{}\n{body}", header.as_str().unwrap())).is_err());
    }
    println!("[DEBUG] Forms rejects foreign text owner, component and version");
}

#[test]
fn forms_document_contract_rejects_foreign_pack_identity() {
    let bytes = FormsSnapshot::default().encode_pack();
    let (_, body) = store::semio_format::unwrap_binary(&bytes).expect("Pack header");
    for header in vectors()["invalidPackEnvelopes"].as_array().unwrap() {
        let envelope = store::semio_format::parse_preamble_line(header.as_str().unwrap()).unwrap();
        assert!(FormsSnapshot::decode_pack(&store::semio_format::wrap_binary(&envelope, &body)).is_err(), "{header}");
    }
    println!("[DEBUG] Forms rejects foreign Pack owner, component and version");
}
