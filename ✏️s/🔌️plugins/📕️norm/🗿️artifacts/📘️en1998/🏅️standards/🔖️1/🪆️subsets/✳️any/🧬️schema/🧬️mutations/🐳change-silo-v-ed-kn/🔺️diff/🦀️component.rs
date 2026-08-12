//! 🔺️ `change-silo-v-ed-kn` sparse diff construction — writes only `En1998Diff.silo_v_ed_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_v_ed_kn::mutation::ChangeSiloVEdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloVEdKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { silo_v_ed_kn: Some(payload.new_silo_v_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
