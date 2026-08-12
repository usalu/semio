//! ↩ Inverse constructor for `RenameSavedCamera` — reconstructed from BASE state.

use super::mutation::RenameSavedCamera;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &RenameSavedCamera, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.saved_cameras.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![ShootingMutation::RenameSavedCamera(RenameSavedCamera { id: payload.id.clone(), new_label: entry.label.clone() })],
        None => Vec::new(),
    }
}
