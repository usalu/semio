//! ↩️ `change-fire-member-capacity-c` — undo restores BASE's fire member capacity factor.

use super::ChangeFireMemberCapacityC;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFireMemberCapacityC, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireMemberCapacityC(ChangeFireMemberCapacityC { new_fire_member_capacity_c: base.fire_member_capacity_c.clone() })]
}
//#endregion 🔖️Inverse
