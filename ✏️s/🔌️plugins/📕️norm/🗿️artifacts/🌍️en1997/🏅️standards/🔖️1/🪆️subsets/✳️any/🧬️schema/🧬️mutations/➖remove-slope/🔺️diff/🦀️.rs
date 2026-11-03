use crate::remove_slope::RemoveSlope;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997SlopesRows};

pub fn diff(payload: &RemoveSlope, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.slopes.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slope #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { slopes: Some(En1997SlopesRows::removal(&base.slopes, payload.index)), ..Default::default() })
}
