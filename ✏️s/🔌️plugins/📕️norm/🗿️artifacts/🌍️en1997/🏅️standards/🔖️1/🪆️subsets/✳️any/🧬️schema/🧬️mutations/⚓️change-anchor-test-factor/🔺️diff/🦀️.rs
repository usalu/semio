//! 🔺️ `change-anchor-test-factor` diff: patches `test_factor` of the addressed anchor row.

use crate::change_anchor_test_factor::ChangeAnchorTestFactor;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorTestFactor, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_test_factor.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.test_factor == payload.new_test_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { test_factor: Some(payload.new_test_factor), ..Default::default() })), ..Default::default() })
}
