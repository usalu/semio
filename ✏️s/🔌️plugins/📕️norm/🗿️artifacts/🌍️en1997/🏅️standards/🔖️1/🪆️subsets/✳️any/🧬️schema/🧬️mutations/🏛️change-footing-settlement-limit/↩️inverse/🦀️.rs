//! ↩️ `change-footing-settlement-limit` inverse: restores the previous `settlement_limit`.

use crate::change_footing_settlement_limit::ChangeFootingSettlementLimit;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeFootingSettlementLimit, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footing_settlement_limit::ChangeFootingSettlementLimit> {
    Ok(match base.footings.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFootingSettlementLimit { id: payload.id.clone(), new_settlement_limit: row.settlement_limit })],
        None => Vec::new(),
    })
}
