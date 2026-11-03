//! 🎚 `change-factor-override-factor` diff: patches `factor` of the addressed factor override row.

use crate::change_factor_override_factor::ChangeFactorOverrideFactor;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FactorOverrideDelta, En1990FactorOverridePatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeFactorOverrideFactor, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.factor_overrides.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Factor override {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.factor == payload.new_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The factor override already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { factor_overrides: En1990FactorOverrideDelta::modification(&payload.id, En1990FactorOverridePatch { factor: Some(payload.new_factor.clone()), ..Default::default() }), ..Default::default() })
}
