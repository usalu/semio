//! 🔺️ `change-silo-n-rd-kn` sparse diff construction — writes only `En1998Diff.silo_n_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_n_rd_kn::mutation::ChangeSiloNRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloNRdKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { silo_n_rd_kn: Some(payload.new_silo_n_rd_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
