//! 🪢️ `update-tension-component-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateTensionComponentInputs;
use crate::diff::{En1993Diff, En1993TensionComponentDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.tension_components.iter().position(|row| row.id == payload.tension_component.id) {
        Some(index) if base.tension_components[index] == payload.tension_component => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993TensionComponentDelta::removal(&base.tension_components, index);
            replacement.absorb(En1993TensionComponentDelta::insertion(index, payload.tension_component.clone()));
            replacement
        }
        None => En1993TensionComponentDelta::insertion(base.tension_components.len(), payload.tension_component.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { tension_components: delta, ..Default::default() })
}
