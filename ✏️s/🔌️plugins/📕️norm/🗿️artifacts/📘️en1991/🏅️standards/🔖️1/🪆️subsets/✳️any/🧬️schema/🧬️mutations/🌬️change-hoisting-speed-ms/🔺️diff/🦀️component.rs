//! 🔺️ `change-hoisting-speed-ms` — sparse diff construction.

use super::mutation::ChangeHoistingSpeedMS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHoistingSpeedMS, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_hoisting_speed_m_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Hoisting speed ms must be a finite number.", Vec::<String>::new());
    }
    if base.hoisting_speed_m_s == payload.new_hoisting_speed_m_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Hoisting speed ms already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { hoisting_speed_m_s: Some(payload.new_hoisting_speed_m_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
