//! 🔺️ `change-beta-computed` sparse diff.

use crate::change_beta_computed::ChangeBetaComputed;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBetaComputed, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.beta_computed == mutation.new_beta_computed {
        return MutationOutcome::empty().warning("mutation.no-op", "beta_computed already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        beta_computed: Some(mutation.new_beta_computed),
        ..En1990Diff::default()
    })
}
