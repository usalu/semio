//! 🎚 `remove-factor-override` diff: removes the addressed factor override row.

use crate::remove_factor_override::RemoveFactorOverride;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FactorOverrideDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveFactorOverride, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.factor_overrides.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "factor_overrides index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { factor_overrides: En1990FactorOverrideDelta::removal(&base.factor_overrides, index), ..Default::default() })
}
