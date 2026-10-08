//! 🔺️ `remove-part-number-input` — sparse diff construction.

use super::mutation::RemovePartNumberInput;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757PartNumberInputsRows};

//#region 🔖️Diff

pub fn diff(payload: &RemovePartNumberInput, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.part_number_inputs.contains_key(&payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Part-number input \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Iso16757Diff { part_number_inputs: Some(Iso16757PartNumberInputsRows { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() })
}
