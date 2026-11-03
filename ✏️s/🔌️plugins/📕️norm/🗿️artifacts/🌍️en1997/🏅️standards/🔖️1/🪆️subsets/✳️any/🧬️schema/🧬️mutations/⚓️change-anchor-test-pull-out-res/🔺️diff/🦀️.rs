//! 🔺️ `change-anchor-test-pull-out-res` diff: patches `pull_out_resistance` of the addressed anchor test row.

use crate::change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997AnchorsTestsPatch, En1997AnchorsTestsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorTestPullOutRes, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_pull_out_resistance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.anchor_id), vec![payload.anchor_id.clone()]);
    };
    let Some(r1) = r0.tests.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor-test {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.pull_out_resistance == payload.new_pull_out_resistance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor test already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.anchor_id, En1997AnchorsPatch { tests: En1997AnchorsTestsRows::modification(&payload.id, En1997AnchorsTestsPatch { pull_out_resistance: Some(payload.new_pull_out_resistance), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
