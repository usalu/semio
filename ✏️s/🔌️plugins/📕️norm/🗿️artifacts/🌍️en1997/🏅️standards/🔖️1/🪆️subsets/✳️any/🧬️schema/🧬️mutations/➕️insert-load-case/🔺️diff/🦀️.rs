use crate::insert_load_case::InsertLoadCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsPatch, En1997FootingsRows, En1997FootingsLoadCasesRows};

pub fn diff(payload: &InsertLoadCase, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.footings.iter().find(|row| row.id == payload.footing_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.footing_id), vec![payload.footing_id.clone()]);
    };
    if parent.load_cases.iter().any(|existing| existing.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Footing load case id {} already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    if payload.index > parent.load_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.load_cases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.footing_id, En1997FootingsPatch { load_cases: En1997FootingsLoadCasesRows::insertion(payload.index, payload.load_case.clone()), ..Default::default() })), ..Default::default() })
}
