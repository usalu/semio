//! 📏 `remove-load-dispersal` diff: removes the addressed load dispersal row.

use crate::remove_load_dispersal::RemoveLoadDispersal;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LoadDispersalDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveLoadDispersal, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.load_dispersals.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load dispersal {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { load_dispersals: En1991LoadDispersalDelta::removal(&base.load_dispersals, index), ..Default::default() })
}
