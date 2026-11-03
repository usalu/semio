use crate::remove_load_case::RemoveLoadCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsPatch, En1997FootingsRows, En1997FootingsLoadCasesRows};

pub fn diff(payload: &RemoveLoadCase, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.footings.iter().find(|row| row.id == payload.footing_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.footing_id), vec![payload.footing_id.clone()]);
    };
    if payload.index >= parent.load_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load-case #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.footing_id, En1997FootingsPatch { load_cases: En1997FootingsLoadCasesRows::removal(&parent.load_cases, payload.index), ..Default::default() })), ..Default::default() })
}
