use crate::remove_pile::RemovePile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesRows};

pub fn diff(payload: &RemovePile, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.piles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::removal(&base.piles, payload.index)), ..Default::default() })
}
