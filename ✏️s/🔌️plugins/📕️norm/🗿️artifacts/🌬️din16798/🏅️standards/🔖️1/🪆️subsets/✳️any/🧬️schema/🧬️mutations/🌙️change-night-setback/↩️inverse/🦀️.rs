//! ↩️ `change-night-setback` inverse.
use super::ChangeNightSetback;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(_payload: &ChangeNightSetback, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeNightSetback(ChangeNightSetback { new_night_setback_k: base.night_setback_k })]
}
