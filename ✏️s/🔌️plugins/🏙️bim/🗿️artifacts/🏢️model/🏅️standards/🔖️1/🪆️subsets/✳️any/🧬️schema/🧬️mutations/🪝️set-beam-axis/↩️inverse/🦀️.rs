//! ↩️ Inverse of `SetBeamAxis`: an absolute `SetBeamAxis` back to the base axis, none when the beam is absent.

use super::SetBeamAxis;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetBeamAxis, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.beams.get(&payload.id) {
        Some(beam) => vec![ModelMutation::SetBeamAxis(SetBeamAxis { id: payload.id.clone(), axis: beam.axis.clone() })],
        None => Vec::new(),
    }
}
