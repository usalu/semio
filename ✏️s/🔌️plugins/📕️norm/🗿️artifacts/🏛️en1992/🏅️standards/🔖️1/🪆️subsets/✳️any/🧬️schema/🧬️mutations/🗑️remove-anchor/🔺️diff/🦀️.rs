use crate::diff::{En1992AnchorList, En1992Diff};
use super::RemoveAnchor;
use crate::En1992Snapshot;

pub fn diff(payload: &RemoveAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let anchors: Vec<_> = base.anchors.iter().filter(|a| a.id != payload.anchor_id).cloned().collect();
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorList { values: anchors }), ..Default::default() })
}
