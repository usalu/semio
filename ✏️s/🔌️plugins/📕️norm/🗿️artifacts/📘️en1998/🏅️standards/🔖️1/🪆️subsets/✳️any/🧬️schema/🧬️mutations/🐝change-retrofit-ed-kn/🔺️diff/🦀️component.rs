//! 🔺️ `change-retrofit-ed-kn` sparse diff construction — writes only `En1998Diff.retrofit_e_d_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::mutation::ChangeRetrofitEDKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitEDKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { retrofit_e_d_kn: Some(payload.new_retrofit_e_d_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
