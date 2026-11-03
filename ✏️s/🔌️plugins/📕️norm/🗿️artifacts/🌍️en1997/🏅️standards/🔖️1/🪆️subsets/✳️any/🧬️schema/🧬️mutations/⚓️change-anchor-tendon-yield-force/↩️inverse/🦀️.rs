//! ↩️ `change-anchor-tendon-yield-force` inverse: restores the previous `tendon_yield_force`.

use crate::change_anchor_tendon_yield_force::ChangeAnchorTendonYieldForce;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorTendonYieldForce, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_tendon_yield_force::ChangeAnchorTendonYieldForce> {
    Ok(match base.anchors.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeAnchorTendonYieldForce { id: payload.id.clone(), new_tendon_yield_force: row.tendon_yield_force })],
        None => Vec::new(),
    })
}
