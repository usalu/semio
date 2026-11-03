//! 🧵 `insert-prestress-steel` diff: inserts the prestressing steel row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_prestress_steel::InsertPrestressSteel;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PrestressSteelsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertPrestressSteel, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    if base.prestress_steels.iter().any(|existing| existing.id == payload.prestress_steel.id) {
        let key = payload.prestress_steel.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Prestressing steel {key} already exists."), [key]);
    }
    if payload.index > base.prestress_steels.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.prestress_steels.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { prestress_steels: Some(En1992PrestressSteelsRows::insertion(payload.index, payload.prestress_steel.clone())), ..Default::default() })
}
