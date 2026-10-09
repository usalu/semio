//! ↩️ Inverse of `SetElementStorey`: an absolute `SetElementStorey` back to the storey the element stood on in the base, none when the element is absent or stands on no storey.

use super::super::elements;
use super::SetElementStorey;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetElementStorey, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match elements::storey_of(base, &payload.id) {
        Some(storey) => vec![ModelMutation::SetElementStorey(SetElementStorey { id: payload.id.clone(), storey })],
        None => Vec::new(),
    }
}
