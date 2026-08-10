//! ↩️ Inverse of `SetSnapshot`.
use crate::artifacts::playground::mutations::PlaygroundMutation;
use crate::artifacts::playground::PlaygroundSnapshot;

/// ↩️ Restores the pre-mutation snapshot.
pub fn inverse(base: &PlaygroundSnapshot) -> Vec<PlaygroundMutation> {
    vec![PlaygroundMutation::SetSnapshot { snapshot: base.clone() }]
}
