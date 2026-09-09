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
