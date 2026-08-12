//! 🔺️ `change-consequence-class` — sparse diff construction; writes only
//! `En1990Diff.consequence_class`.

use super::mutation::ChangeConsequenceClass;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeConsequenceClass, _base: &En1990Snapshot) -> En1990Diff {
    En1990Diff { consequence_class: Some(payload.new_consequence_class), ..Default::default() }
}
//#endregion 🔖️Diff
