//! 🔺️ `change-hr-m-dot-kg-s` sparse diff construction — writes only `Din16798Diff.hr_m_dot_kg_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_m_dot_kg_s::ChangeHrMDotKgS;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrMDotKgS, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_m_dot_kg_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery mass flow rate must be a finite number, got {}.", payload.new_hr_m_dot_kg_s), Vec::<String>::new());
    }
    if base.hr_m_dot_kg_s == payload.new_hr_m_dot_kg_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery mass flow rate is already {}.", payload.new_hr_m_dot_kg_s));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_m_dot_kg_s: Some(payload.new_hr_m_dot_kg_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
