//! 🔺️ `change-part-number-input` — sparse diff construction.

use super::mutation::ChangePartNumberInput;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePartNumberInput, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut inputs = base.part_number_inputs.clone();
    inputs.insert(payload.key.clone(), payload.new_value.clone());
    Iso16757Diff { part_number_inputs: Some(inputs), ..Default::default() }
}
//#endregion 🔖️Diff
