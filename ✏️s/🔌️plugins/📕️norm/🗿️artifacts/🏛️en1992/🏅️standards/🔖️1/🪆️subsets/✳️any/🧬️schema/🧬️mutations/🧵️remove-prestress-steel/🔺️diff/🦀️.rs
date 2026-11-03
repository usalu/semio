//! 🧵 `remove-prestress-steel` diff: removes the addressed prestressing steel row.

use crate::remove_prestress_steel::RemovePrestressSteel;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992PrestressSteelsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemovePrestressSteel, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {

    let index = payload.index;
    if index >= base.prestress_steels.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "prestress_steels index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { prestress_steels: Some(En1992PrestressSteelsRows::removal(&base.prestress_steels, index)), ..Default::default() })
}
