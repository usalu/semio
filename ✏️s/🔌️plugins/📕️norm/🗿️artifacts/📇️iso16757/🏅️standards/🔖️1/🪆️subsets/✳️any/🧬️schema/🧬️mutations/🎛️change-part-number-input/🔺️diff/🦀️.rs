//! 🔺️ `change-part-number-input` — sparse diff construction.

use super::mutation::ChangePartNumberInput;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757PartNumberInputsRows, Iso16757PartNumberInputsEntry};

//#region 🔖️Diff

pub fn diff(payload: &ChangePartNumberInput, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.part_number_inputs.get(&payload.key) == Some(&payload.new_value) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Part-number input \"{}\" already has this value.", payload.key));
    }
    let entry = Iso16757PartNumberInputsEntry { key: payload.key.clone(), value: payload.new_value.clone() };
    let rows = if base.part_number_inputs.contains_key(&payload.key) {
        Iso16757PartNumberInputsRows { modified: vec![entry], ..Default::default() }
    } else {
        Iso16757PartNumberInputsRows { added: vec![entry], ..Default::default() }
    };
    protocol::MutationOutcome::new(Iso16757Diff { part_number_inputs: Some(rows), ..Default::default() })
}
