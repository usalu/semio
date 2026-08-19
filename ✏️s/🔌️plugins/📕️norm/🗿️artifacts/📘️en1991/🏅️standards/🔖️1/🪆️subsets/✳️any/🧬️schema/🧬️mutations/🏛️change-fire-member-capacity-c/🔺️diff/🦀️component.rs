//! 🔺️ `change-fire-member-capacity-c` — sparse diff construction.

use super::mutation::ChangeFireMemberCapacityC;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFireMemberCapacityC, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_fire_member_capacity_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fire member capacity c must be a finite number.", Vec::<String>::new());
    }
    if base.fire_member_capacity_c == payload.new_fire_member_capacity_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire member capacity c already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_member_capacity_c: Some(payload.new_fire_member_capacity_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
