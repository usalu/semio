use super::*;

/// 🧬️ `Wave C` (SEMANTIC-MUTATIONS-OVERHAUL) removed this facet's `🔖️Constructors` region — a
/// dead helper set parametrized over the old generic per-collection add/remove/patch wrapper,
/// with zero external callers
/// (superseded by the semantic `🧬️mutations` triads' own diff leaves) — per the ticket's banned-
/// vocabulary final sweep. `apply_to_artifact` (the one real, still-live function in this file)
/// keeps its own coverage here instead.
#[semio_framework_async_macros::async_test]
async fn apply_to_artifact_applies_a_scalar_field() {
    let artifact = ProgramArtifact::default();
    let mut renamed_meta = artifact.meta.clone();
    renamed_meta.title = "Renamed".into();
    let diff = ProgramDiff { meta: Some(renamed_meta.clone()), ..Default::default() };
    let next = diff.apply_to_artifact(&artifact).expect("valid artifact diff");
    assert_eq!(next.meta.title, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn apply_to_artifact_full_replacement_wins_over_field_entries() {
    let artifact = ProgramArtifact::default();
    let mut replacement = artifact.clone();
    replacement.schema = "s.architect.program@2".into();
    let diff = ProgramDiff { artifact: Some(Box::new(replacement.clone())), schema: Some("ignored".into()), ..Default::default() };
    let next = diff.apply_to_artifact(&artifact).expect("valid artifact diff");
    assert_eq!(next, replacement);
}

#[test]
fn architect_native_sparse_diff_preserves_declared_roles_and_exact_text() {
    use semio_framework_dsl_record::{parse_exact, print, JoinMode, ParseOptions};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔗️native-roles/🧫️fixtures/🔣️.json")).unwrap();
    let delta: ProgramStakeholdersDelta = serde_json::from_value(fixture["delta"].clone()).unwrap();
    for value in [ProgramDiff::default(), ProgramDiff { stakeholders: Some(delta), ..Default::default() }] {
        let record = value.__dsl_to_record();
        let spec = ProgramDiff::__dsl_spec();
        for mode in [JoinMode::Inline, JoinMode::Document] {
            let text = print(&record, &spec, mode);
            let parsed = parse_exact(&text, &spec, &ParseOptions::default()).unwrap();
            assert_eq!(ProgramDiff::__dsl_from_record(&parsed).unwrap(), value);
            assert!(parse_exact(&format!("{text} unowned 17"), &spec, &ParseOptions::default()).is_err());
        }
    }
    eprintln!("[DEBUG] Architect native sparse delta roles preserve independent serde fixture and exact terminal grammar");
}
