//! 🔺️ `change-fire-member-capacity-c` — sparse diff construction.

use super::mutation::ChangeFireMemberCapacityC;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireMemberCapacityC, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { fire_member_capacity_c: Some(payload.new_fire_member_capacity_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
