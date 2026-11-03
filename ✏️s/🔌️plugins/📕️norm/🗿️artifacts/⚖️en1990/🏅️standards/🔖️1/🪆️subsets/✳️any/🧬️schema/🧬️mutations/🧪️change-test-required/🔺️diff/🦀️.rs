//! 🧪 `change-test-required` diff: patches `required` of the addressed test series row.

use crate::change_test_required::ChangeTestRequired;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990TestSeriesDelta, En1990TestSeriesPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeTestRequired, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_required.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.test_series.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Test series {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.required == payload.new_required {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The test series already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { test_series: En1990TestSeriesDelta::modification(&payload.id, En1990TestSeriesPatch { required: Some(payload.new_required), ..Default::default() }), ..Default::default() })
}
