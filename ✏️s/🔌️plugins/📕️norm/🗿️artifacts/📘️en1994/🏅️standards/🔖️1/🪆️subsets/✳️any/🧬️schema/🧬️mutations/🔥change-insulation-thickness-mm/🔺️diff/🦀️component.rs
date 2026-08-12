//! 🔺️ `change-insulation-thickness-mm` — sparse diff construction.

use super::mutation::ChangeInsulationThicknessMm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeInsulationThicknessMm, _base: &En1994Snapshot) -> En1994Diff {
    En1994Diff { insulation_thickness_mm: Some(payload.new_insulation_thickness_mm), ..Default::default() }
}
//#endregion 🔖️Diff
