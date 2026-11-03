//! ↩️ `change-anchor-test-factor` inverse: restores the previous `test_factor`.

use crate::change_anchor_test_factor::ChangeAnchorTestFactor;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorTestFactor, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_test_factor::ChangeAnchorTestFactor> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorTestFactor { id: payload.id.clone(), new_test_factor: row.test_factor })],
        None => Vec::new(),
    })
}
