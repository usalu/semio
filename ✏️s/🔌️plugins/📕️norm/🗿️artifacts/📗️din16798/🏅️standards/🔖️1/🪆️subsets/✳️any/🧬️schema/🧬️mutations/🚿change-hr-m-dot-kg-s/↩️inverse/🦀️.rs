//! ↩️ `change-hr-m-dot-kg-s` inverse — restores the pre-change `hr_m_dot_kg_s` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din16798::mutations::change_hr_m_dot_kg_s::ChangeHrMDotKgS;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeHrMDotKgS, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeHrMDotKgS(ChangeHrMDotKgS { new_hr_m_dot_kg_s: base.hr_m_dot_kg_s.clone() })]
}
//#endregion 🔖️Inverse
