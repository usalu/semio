//! 🔺️ `change-accidental-mass-t` — sparse diff construction.

use super::mutation::ChangeAccidentalMassT;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAccidentalMassT, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { accidental_mass_t: Some(payload.new_accidental_mass_t.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
