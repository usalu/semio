//! ⚙ `insert-fatigue-model` diff: inserts the fatigue model row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_fatigue_model::InsertFatigueModel;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FatigueModelDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertFatigueModel, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fatigue_models.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue model {key} already exists."), [key]);
    }
    if payload.index > base.fatigue_models.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.fatigue_models.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { fatigue_models: En1991FatigueModelDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
