use crate::insert_anchor_test::InsertAnchorTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997AnchorsPatch, En1997AnchorsRows, En1997AnchorsTestsRows};

pub fn diff(payload: &InsertAnchorTest, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.anchor_id), vec![payload.anchor_id.clone()]);
    };
    if parent.tests.iter().any(|existing| existing.id == payload.anchor_test.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Anchor test id {} already exists.", payload.anchor_test.id), [payload.anchor_test.id.clone()]);
    }
    if payload.index > parent.tests.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.tests.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.anchor_id, En1997AnchorsPatch { tests: En1997AnchorsTestsRows::insertion(payload.index, payload.anchor_test.clone()), ..Default::default() })), ..Default::default() })
}
