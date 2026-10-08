//! ↩️ Inverse of `SetBeam`: an absolute `SetBeam` restoring the base value of exactly the fields the forward really changes,
//! none when the beam is absent or nothing would change.

use super::SetBeam;
use crate::{ModelMutation, ModelSnapshot};

fn restored<T: Clone + PartialEq>(value: &Option<T>, current: &T) -> Option<T> {
    value.as_ref().filter(|next| *next != current).map(|_| current.clone())
}

pub fn inverse(payload: &SetBeam, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return Vec::new();
    };
    let restore = SetBeam {
        id: payload.id.clone(),
        beam_type: restored(&payload.beam_type, &beam.beam_type),
        start: restored(&payload.start, &beam.start),
        end: restored(&payload.end, &beam.end),
        top_offset: restored(&payload.top_offset, &beam.top_offset),
        name: restored(&payload.name, &beam.name),
    };
    let nothing = SetBeam { id: payload.id.clone(), beam_type: None, start: None, end: None, top_offset: None, name: None };
    if restore == nothing {
        return Vec::new();
    }
    vec![ModelMutation::SetBeam(restore)]
}
