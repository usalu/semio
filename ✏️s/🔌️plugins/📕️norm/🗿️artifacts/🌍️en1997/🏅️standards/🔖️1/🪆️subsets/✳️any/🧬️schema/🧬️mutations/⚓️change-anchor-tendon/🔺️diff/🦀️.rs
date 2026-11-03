//! 🔺️ `change-anchor-tendon` diff: patches `tendon` of the addressed anchor row.

use crate::change_anchor_tendon::ChangeAnchorTendon;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorTendon, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.tendon == payload.new_tendon {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { tendon: Some(payload.new_tendon), ..Default::default() })), ..Default::default() })
}
