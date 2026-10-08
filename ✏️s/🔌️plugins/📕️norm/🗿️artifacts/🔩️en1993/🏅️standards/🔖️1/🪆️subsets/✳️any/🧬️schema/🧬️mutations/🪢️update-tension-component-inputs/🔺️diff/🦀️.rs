//! 🪢️ `update-tension-component-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateTensionComponentInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993TensionComponentEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.tension_components.iter().position(|row| row.id == payload.tension_component.id) {
        Some(index) if base.tension_components[index] == payload.tension_component => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993TensionComponentEdit::replace(index, payload.tension_component.id.clone(), payload.tension_component.clone()),
        None => En1993TensionComponentEdit::insert(base.tension_components.len(), payload.tension_component.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { tension_components: vec![edit], ..Default::default() })
}
