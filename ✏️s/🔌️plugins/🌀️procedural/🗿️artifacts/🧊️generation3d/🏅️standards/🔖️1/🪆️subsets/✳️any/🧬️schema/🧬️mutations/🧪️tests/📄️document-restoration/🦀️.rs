//! 📄️ Original semantic mutation authority for imported generation selection and preview restoration.

use crate::standards::v1::subsets::any::schema::mutations::{apply_generation3d_mutation,inverse_generation3d_mutation,Generation3dMutation};

use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::editor::generation3d::Generation3dPlayApp;
use semio_framework_plugin::ArtifactEditor;
use protocol::{Mutation, OpBinary, OpText};

#[test]
fn document_restoration_selection_preview_mutation_algebra_matches_neutral_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️document-restoration/🔣️.json")).unwrap();
    let mut base = Generation3dSnapshotRead::new(<Generation3dPlayApp as ArtifactEditor>::initial_snapshot());
    let before = serde_json::to_string(&fixture["before"]).unwrap();
    let state: semio_framework_artifact_playbook_playbook::GenerationPlayState = semio_framework_pack_json::from_json_str(&before, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    std::mem::replace(&mut base.generation, state.into()).retire_cold();
    let mut checks = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let wire = serde_json::to_string(&case["mutation"]).unwrap();
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(fixture["maximumDecodeBytes"].as_u64().unwrap() as usize, &mut accepted);
        let parsed = semio_framework_pack_json::from_json_str_controlled::<Generation3dMutation>(&wire, semio_framework_pack_json::JsonMemberPolicy::Reject, &mut control);
        let Ok(mutation) = parsed else { checks.push((case["id"].as_str().unwrap().to_owned(), false)); continue };
        let binary = mutation.encode_op().ok().and_then(|bytes| Generation3dMutation::decode_op(&bytes).ok());
        let text = Generation3dMutation::parse_op(&mutation.print_op()).ok();
        let codec_parity = binary.as_ref() == Some(&mutation) && text.as_ref() == Some(&mutation);
        if let Some(value) = binary { value.retire_cold(); }
        if let Some(value) = text { value.retire_cold(); }
        let (delta, messages) = mutation.diff(&base).into_parts();
        let sparse = delta.artifact.is_none() && delta.host_snapshot.is_none();
        let no_op = case["status"] != "no-op" || delta.generation.is_none();
        delta.retire_cold();
        let inverse = inverse_generation3d_mutation(&base, &mutation).unwrap();
        let mut current = Generation3dSnapshotRead::new((*base).clone());
        let applied = apply_generation3d_mutation(&mut current, &mutation);
        let expected_rejection = case["status"] == "rejected";
        let publication = applied.is_err() == expected_rejection;
        let rejection = !expected_rejection || (current == base && messages.iter().any(|message| message.code.0 == case["faultCode"].as_str().unwrap()));
        let mut expected = fixture["before"].clone();
        expected["selectedGenerationId"] = case["selectedGenerationId"].clone();
        expected["previewText"] = case["previewText"].clone();
        let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&current.generation)).unwrap();
        let state_parity = observed == expected && current.host_snapshot == base.host_snapshot;
        if !expected_rejection { for value in &inverse { apply_generation3d_mutation(&mut current, value).unwrap(); } }
        let inverse_parity = current == base;
        for value in inverse { value.retire_cold(); }
        mutation.retire_cold();
        checks.push((case["id"].as_str().unwrap().to_owned(), codec_parity && sparse && no_op && publication && rejection && state_parity && inverse_parity));
    }
    eprintln!("[DEBUG] original document restoration mutation algebra checks={checks:?}");
    assert!(checks.iter().all(|(_, agrees)| *agrees), "original selection/preview semantic mutation authority must satisfy every neutral algebra case");
}
