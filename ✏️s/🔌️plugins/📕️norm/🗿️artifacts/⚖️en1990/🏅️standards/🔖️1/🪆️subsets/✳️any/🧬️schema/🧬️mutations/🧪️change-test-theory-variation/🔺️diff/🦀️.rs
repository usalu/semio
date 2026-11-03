//! 🧪 `change-test-theory-variation` diff: patches `theory_variation` of the addressed test series row.

use crate::change_test_theory_variation::ChangeTestTheoryVariation;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TestSeriesDelta, En1990TestSeriesPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeTestTheoryVariation, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_theory_variation.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.test_series.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Test series {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.theory_variation == payload.new_theory_variation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The test series already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { test_series: En1990TestSeriesDelta::modification(&payload.id, En1990TestSeriesPatch { theory_variation: Some(payload.new_theory_variation), ..Default::default() }), ..Default::default() })
}
