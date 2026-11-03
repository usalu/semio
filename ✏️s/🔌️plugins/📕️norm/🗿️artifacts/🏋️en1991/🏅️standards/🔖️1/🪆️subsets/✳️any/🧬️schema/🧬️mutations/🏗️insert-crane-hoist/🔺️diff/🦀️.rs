//! 🏗 `insert-crane-hoist` diff: inserts the crane hoist row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_crane_hoist::InsertCraneHoist;
use semio_s_artifact_norm_en1991_diff::{En1991CraneHoistDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertCraneHoist, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.crane_hoists.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Crane hoist {key} already exists."), [key]);
    }
    if payload.index > base.crane_hoists.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.crane_hoists.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { crane_hoists: En1991CraneHoistDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
