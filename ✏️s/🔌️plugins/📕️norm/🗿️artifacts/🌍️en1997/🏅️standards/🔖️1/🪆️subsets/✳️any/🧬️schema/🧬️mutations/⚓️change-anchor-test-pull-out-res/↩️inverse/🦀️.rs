//! ↩️ `change-anchor-test-pull-out-res` inverse: restores the previous `pull_out_resistance`.

use crate::change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorTestPullOutRes, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.anchor_id).and_then(|row| row.tests.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeAnchorTestPullOutRes { anchor_id: payload.anchor_id.clone(), id: payload.id.clone(), new_pull_out_resistance: row.pull_out_resistance })],
        None => Vec::new(),
    })
}
