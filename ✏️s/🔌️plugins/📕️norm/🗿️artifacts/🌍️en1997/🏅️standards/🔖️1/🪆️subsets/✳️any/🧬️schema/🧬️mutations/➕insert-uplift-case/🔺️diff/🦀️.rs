use crate::insert_uplift_case::InsertUpliftCase;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997UpliftCasesRows};

pub fn diff(payload: &InsertUpliftCase, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.uplift_cases.iter().any(|existing| existing.id == payload.uplift_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("UpliftCase id {} already exists.", payload.uplift_case.id), [payload.uplift_case.id.clone()]);
    }
    if payload.index > base.uplift_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.uplift_cases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { uplift_cases: Some(En1997UpliftCasesRows::insertion(payload.index, payload.uplift_case.clone())), ..Default::default() })
}
