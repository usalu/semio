//! ↩️ Inverse of `RenameElement`: an absolute `RenameElement` back to the base name, none when the element is absent.

use super::super::elements;
use super::RenameElement;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RenameElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match elements::rename(base, &payload.id, &payload.name) {
        Some((current, _)) => vec![ModelMutation::RenameElement(RenameElement { id: payload.id.clone(), name: current })],
        None => Vec::new(),
    }
}
