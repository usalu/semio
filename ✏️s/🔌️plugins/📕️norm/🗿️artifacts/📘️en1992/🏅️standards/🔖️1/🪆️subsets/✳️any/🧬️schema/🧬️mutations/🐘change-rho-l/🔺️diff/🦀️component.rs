//! 🔺️ `change-rho-l` sparse diff construction — writes only `En1992Diff.rho_l` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_rho_l::mutation::ChangeRhoL;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRhoL, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { rho_l: Some(payload.new_rho_l.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
