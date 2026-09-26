//! 🔺️ `change-beta-computed` sparse diff.

use super::ChangeBetaComputed;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBetaComputed, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.beta_computed == mutation.new_beta_computed {
        return MutationOutcome::empty().warn("mutation.no-op", "beta_computed already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        beta_computed: Some(mutation.new_beta_computed),
        ..En1990Diff::default()
    })
}
