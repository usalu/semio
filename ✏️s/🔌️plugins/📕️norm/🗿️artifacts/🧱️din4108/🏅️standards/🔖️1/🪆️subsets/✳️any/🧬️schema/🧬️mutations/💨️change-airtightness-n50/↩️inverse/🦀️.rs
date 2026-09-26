//! ↩️ `change-airtightness-n50` inverse.

use super::ChangeAirtightnessN50;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeAirtightnessN50, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: base.airtightness_n50 })]
}
