//! ↩️ `change-accidental-mass-t` — undo restores BASE's accidental impact mass.

use super::mutation::ChangeAccidentalMassT;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAccidentalMassT, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAccidentalMassT(ChangeAccidentalMassT { new_accidental_mass_t: base.accidental_mass_t.clone() })]
}
//#endregion 🔖️Inverse
