//! 🔺️ `remove-part-number-input` — sparse diff construction.

use super::mutation::RemovePartNumberInput;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemovePartNumberInput, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut inputs = base.part_number_inputs.clone();
    inputs.remove(&payload.key);
    Iso16757Diff { part_number_inputs: Some(inputs), ..Default::default() }
}
//#endregion 🔖️Diff
