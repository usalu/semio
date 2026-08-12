//! 🔺️ `change-hr-m-dot-kg-s` sparse diff construction — writes only `Din16798Diff.hr_m_dot_kg_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_m_dot_kg_s::mutation::ChangeHrMDotKgS;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrMDotKgS, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { hr_m_dot_kg_s: Some(payload.new_hr_m_dot_kg_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
