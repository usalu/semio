use crate::diff::{En1992AnchorList, En1992Diff};
use super::InsertAnchor;
use crate::En1992Snapshot;

pub fn diff(payload: &InsertAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let mut anchors = base.anchors.clone();
    let at = payload.index.min(anchors.len());
    anchors.insert(at, payload.anchor.clone());
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorList { values: anchors }), ..Default::default() })
}
