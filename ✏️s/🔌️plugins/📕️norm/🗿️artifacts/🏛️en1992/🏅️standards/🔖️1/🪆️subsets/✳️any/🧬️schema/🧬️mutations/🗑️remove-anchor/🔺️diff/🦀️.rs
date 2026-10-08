use crate::diff::{En1992Diff, En1992AnchorsRows};
use super::RemoveAnchor;
use crate::En1992Snapshot;

pub fn diff(payload: &RemoveAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !base.anchors.iter().any(|anchor| anchor.id == payload.anchor_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows { removed: vec![payload.anchor_id.clone()], ..Default::default() }), ..Default::default() })
}
