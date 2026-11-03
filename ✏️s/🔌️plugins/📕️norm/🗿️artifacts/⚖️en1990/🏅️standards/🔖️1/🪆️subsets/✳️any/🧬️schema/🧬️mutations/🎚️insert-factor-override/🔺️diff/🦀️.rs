//! 🎚 `insert-factor-override` diff: inserts the factor override row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_factor_override::InsertFactorOverride;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990FactorOverrideDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertFactorOverride, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.factor_overrides.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Factor override {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.factor_overrides.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { factor_overrides: En1990FactorOverrideDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the factor override list; inserted at {index}.", payload.index.unwrap_or(index)))
}
