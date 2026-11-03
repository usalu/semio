//! 🌾 `remove-bulk-solid` diff: removes the addressed bulk solid row.

use crate::remove_bulk_solid::RemoveBulkSolid;
use semio_s_artifact_norm_en1991_diff::{En1991BulkSolidDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveBulkSolid, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.bulk_solids.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bulk solid {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { bulk_solids: En1991BulkSolidDelta::removal(&base.bulk_solids, index), ..Default::default() })
}
