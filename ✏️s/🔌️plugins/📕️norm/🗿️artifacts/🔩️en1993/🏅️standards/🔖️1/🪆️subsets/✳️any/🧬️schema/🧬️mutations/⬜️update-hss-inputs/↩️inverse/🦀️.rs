//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateHssInputs;
use crate::mutations::remove_load_case;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateHssInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.load_cases.iter().find(|x| x.id == payload.load_case.id) {
        vec![En1993Mutation::UpdateHssInputs(UpdateHssInputs { load_case: prior.clone()  })]
    } else {
        vec![En1993Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { index: base.load_cases.len()  })]
    }
}
