//! ↩️ `change-settlement-limit-mm` inverse — restores the pre-change `settlement_limit_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_settlement_limit_mm::mutation::ChangeSettlementLimitMm;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSettlementLimitMm, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeSettlementLimitMm(ChangeSettlementLimitMm { new_settlement_limit_mm: base.settlement_limit_mm.clone() })]
}
//#endregion 🔖️Inverse
