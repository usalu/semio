//! ⚙ `remove-snow-guard` diff: removes the addressed snow guard row.

use crate::remove_snow_guard::RemoveSnowGuard;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SnowGuardDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveSnowGuard, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.snow_guards.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Snow guard {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { snow_guards: En1991SnowGuardDelta::removal(&base.snow_guards, index), ..Default::default() })
}
