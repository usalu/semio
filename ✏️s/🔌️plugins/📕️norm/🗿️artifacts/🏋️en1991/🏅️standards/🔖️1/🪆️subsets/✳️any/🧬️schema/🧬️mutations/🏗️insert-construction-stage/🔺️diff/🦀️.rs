//! 🏗️ `insert-construction-stage` diff: inserts the construction stage row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_construction_stage::InsertConstructionStage;
use semio_s_artifact_norm_en1991_diff::{En1991ConstructionStageDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertConstructionStage, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.construction_stages.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("construction stage {key} already exists."), [key]);
    }
    if payload.index > base.construction_stages.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.construction_stages.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { construction_stages: En1991ConstructionStageDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
