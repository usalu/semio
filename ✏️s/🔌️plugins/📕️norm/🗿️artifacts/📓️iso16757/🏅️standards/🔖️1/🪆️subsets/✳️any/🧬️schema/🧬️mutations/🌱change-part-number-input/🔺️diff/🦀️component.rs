//! 🔺️ `change-part-number-input` — sparse diff construction.

use super::mutation::ChangePartNumberInput;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangePartNumberInput, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.part_number_inputs.get(&payload.key) == Some(&payload.new_value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Part-number input \"{}\" already has this value.", payload.key));
    }
    let mut inputs = base.part_number_inputs.clone();
    inputs.insert(payload.key.clone(), payload.new_value.clone());
    protocol::MutationOutcome::new(Iso16757Diff { part_number_inputs: Some(inputs), ..Default::default() })
}
//#endregion 🔖️Diff
