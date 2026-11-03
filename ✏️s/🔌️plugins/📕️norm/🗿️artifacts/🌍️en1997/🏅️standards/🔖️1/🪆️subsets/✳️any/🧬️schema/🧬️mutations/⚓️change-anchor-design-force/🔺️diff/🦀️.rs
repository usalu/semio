//! 🔺️ `change-anchor-design-force` diff: patches `design_force` of the addressed anchor row.

use crate::change_anchor_design_force::ChangeAnchorDesignForce;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorDesignForce, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_design_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.design_force == payload.new_design_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { design_force: Some(payload.new_design_force), ..Default::default() })), ..Default::default() })
}
