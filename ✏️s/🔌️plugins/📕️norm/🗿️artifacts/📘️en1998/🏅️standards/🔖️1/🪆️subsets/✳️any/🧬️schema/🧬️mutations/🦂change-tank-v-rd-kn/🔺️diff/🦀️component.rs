//! 🔺️ `change-tank-v-rd-kn` sparse diff construction — writes only `En1998Diff.tank_v_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_v_rd_kn::mutation::ChangeTankVRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankVRdKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tank_v_rd_kn: Some(payload.new_tank_v_rd_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
