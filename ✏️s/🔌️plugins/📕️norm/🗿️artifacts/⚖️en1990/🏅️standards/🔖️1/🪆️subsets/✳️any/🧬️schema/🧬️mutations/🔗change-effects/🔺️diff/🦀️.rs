//! 🔺️ `change-effects` sparse diff.

use super::ChangeEffects;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeEffects, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.effects == &mutation.new_effects {
        return MutationOutcome::empty().warn("mutation.no-op", "effects already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        effects: Some(mutation.new_effects.clone()),
        ..En1990Diff::default()
    })
}
