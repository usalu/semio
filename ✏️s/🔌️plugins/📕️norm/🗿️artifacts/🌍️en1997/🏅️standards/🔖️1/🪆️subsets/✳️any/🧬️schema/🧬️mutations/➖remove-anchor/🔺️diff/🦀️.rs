use crate::remove_anchor::RemoveAnchor;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997AnchorsRows};

pub fn diff(payload: &RemoveAnchor, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.anchors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::removal(&base.anchors, payload.index)), ..Default::default() })
}
