//! 🔺️ `remove-transverse-stiffener` diff.

use crate::remove_transverse_stiffener::RemoveTransverseStiffener;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999TransverseStiffenersRows};

pub fn diff(payload: &RemoveTransverseStiffener, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.transverse_stiffeners.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown transverse stiffener id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { transverse_stiffeners: Some(En1999TransverseStiffenersRows::removal(&base.transverse_stiffeners, index)), ..Default::default() })
}
