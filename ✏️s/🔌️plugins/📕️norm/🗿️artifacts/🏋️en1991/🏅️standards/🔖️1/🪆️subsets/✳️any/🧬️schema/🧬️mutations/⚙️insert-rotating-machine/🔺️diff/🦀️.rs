//! ⚙ `insert-rotating-machine` diff: inserts the rotating machine row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_rotating_machine::InsertRotatingMachine;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RotatingMachineDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertRotatingMachine, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.rotating_machines.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Rotating machine {key} already exists."), [key]);
    }
    if payload.index > base.rotating_machines.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.rotating_machines.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { rotating_machines: En1991RotatingMachineDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
