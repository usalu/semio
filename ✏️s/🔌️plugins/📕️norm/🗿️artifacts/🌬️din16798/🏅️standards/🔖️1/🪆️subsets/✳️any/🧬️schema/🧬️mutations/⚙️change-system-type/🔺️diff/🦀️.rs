//! 🔺️ `change-system-type` sparse diff construction — writes only `Din16798Diff.system_type` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_system_type::ChangeSystemType;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSystemType, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.system_type == payload.new_system_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ventilation system type is already \"{}\".", payload.new_system_type));
    }
    protocol::MutationOutcome::new(Din16798Diff { system_type: Some(payload.new_system_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
