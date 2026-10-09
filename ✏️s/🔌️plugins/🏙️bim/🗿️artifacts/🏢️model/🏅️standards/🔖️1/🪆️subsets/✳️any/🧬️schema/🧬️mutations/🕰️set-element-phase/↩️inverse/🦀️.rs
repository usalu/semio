//! ↩️ Inverse of `SetElementPhase`: an absolute `SetElementPhase` back to the phase the element was in on the base, none when the element is absent or carries no phase.

use super::super::elements;
use super::SetElementPhase;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetElementPhase, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match elements::phase_of(base, &payload.id) {
        Some(phase) => vec![ModelMutation::SetElementPhase(SetElementPhase { id: payload.id.clone(), phase })],
        None => Vec::new(),
    }
}
