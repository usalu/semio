//! 🔺️ `change-retrofit-rk-kn` sparse diff construction — writes only `En1998Diff.retrofit_r_k_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_r_k_kn::mutation::ChangeRetrofitRKKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitRKKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { retrofit_r_k_kn: Some(payload.new_retrofit_r_k_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
