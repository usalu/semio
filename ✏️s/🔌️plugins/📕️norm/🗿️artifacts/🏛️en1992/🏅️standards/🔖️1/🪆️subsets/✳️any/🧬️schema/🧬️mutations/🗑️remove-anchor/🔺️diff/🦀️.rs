use crate::diff::{En1992Diff, En1992AnchorsRows};
use super::RemoveAnchor;
use crate::En1992Snapshot;

pub fn diff(payload: &RemoveAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(index) = base.anchors.iter().position(|anchor| anchor.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::removal(&base.anchors, index)), ..Default::default() })
}
