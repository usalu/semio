//! 🔺️ `remove-part-number-input` — sparse diff construction.

use super::mutation::RemovePartNumberInput;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemovePartNumberInput, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.part_number_inputs.contains_key(&payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Part-number input \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    }
    let mut inputs = base.part_number_inputs.clone();
    inputs.remove(&payload.key);
    protocol::MutationOutcome::new(Iso16757Diff { part_number_inputs: Some(inputs), ..Default::default() })
}
//#endregion 🔖️Diff
