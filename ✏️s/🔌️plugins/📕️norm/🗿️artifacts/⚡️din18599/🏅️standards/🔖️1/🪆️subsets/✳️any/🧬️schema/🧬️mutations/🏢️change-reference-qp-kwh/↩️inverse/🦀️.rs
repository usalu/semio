//! ↩️ `change-reference-qp-kwh` inverse — restores the pre-change `reference_q_p_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_reference_q_p_kwh::ChangeReferenceQPKwh;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeReferenceQPKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeReferenceQPKwh(ChangeReferenceQPKwh { new_reference_q_p_kwh: base.reference_q_p_kwh })]
}
//#endregion 🔖️Inverse
