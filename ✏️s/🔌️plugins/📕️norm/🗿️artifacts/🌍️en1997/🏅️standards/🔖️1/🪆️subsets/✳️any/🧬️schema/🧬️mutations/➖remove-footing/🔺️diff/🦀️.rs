use crate::remove_footing::RemoveFooting;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsRows};

pub fn diff(payload: &RemoveFooting, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.footings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::removal(&base.footings, payload.index)), ..Default::default() })
}
