//! 🔺️ `change-cursor` sparse diff construction. Document-level scalar setter — no target to be
//! missing (root-scoped `change-<artifact>-<field>` shrink-only allowlist); Warning `no-op` when the
//! cursor is already at that value.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeCursor, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    if base.resolved_up_to == payload.new_resolved_up_to {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Cursor is already at that position.".to_string());
    }
    protocol::MutationOutcome::new(Process3dDiff { resolved_up_to: Some(payload.new_resolved_up_to), ..Default::default() })
}
//#endregion 🔖️Diff
