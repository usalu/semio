//! ⚙ `remove-fatigue-model` diff: removes the addressed fatigue model row.

use crate::remove_fatigue_model::RemoveFatigueModel;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FatigueModelDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveFatigueModel, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.fatigue_models.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue model {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { fatigue_models: En1991FatigueModelDelta::removal(&base.fatigue_models, index), ..Default::default() })
}
