use crate::remove_anchor_test::RemoveAnchorTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997AnchorsPatch, En1997AnchorsRows, En1997AnchorsTestsRows};

pub fn diff(payload: &RemoveAnchorTest, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.anchor_id), vec![payload.anchor_id.clone()]);
    };
    if payload.index >= parent.tests.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor-test #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.anchor_id, En1997AnchorsPatch { tests: En1997AnchorsTestsRows::removal(&parent.tests, payload.index), ..Default::default() })), ..Default::default() })
}
