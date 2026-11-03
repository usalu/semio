//! 🎚 `change-factor-override-value` diff: patches `value` of the addressed factor override row.

use crate::change_factor_override_value::ChangeFactorOverrideValue;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FactorOverrideDelta, En1990FactorOverridePatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeFactorOverrideValue, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.factor_overrides.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Factor override {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.value == payload.new_value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The factor override already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { factor_overrides: En1990FactorOverrideDelta::modification(&payload.id, En1990FactorOverridePatch { value: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
