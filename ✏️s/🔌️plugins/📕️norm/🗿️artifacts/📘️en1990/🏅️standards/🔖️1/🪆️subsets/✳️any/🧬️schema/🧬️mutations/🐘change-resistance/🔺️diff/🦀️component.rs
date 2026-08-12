//! 🔺️ `change-resistance` — sparse diff construction; writes only `En1990Diff.resistance_kn`.

use super::mutation::ChangeResistance;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeResistance, _base: &En1990Snapshot) -> En1990Diff {
    En1990Diff { resistance_kn: Some(payload.new_resistance_kn), ..Default::default() }
}
//#endregion 🔖️Diff
