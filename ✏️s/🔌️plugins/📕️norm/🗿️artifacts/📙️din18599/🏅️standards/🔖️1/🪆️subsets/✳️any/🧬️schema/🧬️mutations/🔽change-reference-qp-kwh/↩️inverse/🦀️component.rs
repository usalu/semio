//! ↩️ `change-reference-qp-kwh` inverse — restores the pre-change `reference_q_p_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeReferenceQPKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeReferenceQPKwh(ChangeReferenceQPKwh { new_reference_q_p_kwh: base.reference_q_p_kwh.clone() })]
}
//#endregion 🔖️Inverse
