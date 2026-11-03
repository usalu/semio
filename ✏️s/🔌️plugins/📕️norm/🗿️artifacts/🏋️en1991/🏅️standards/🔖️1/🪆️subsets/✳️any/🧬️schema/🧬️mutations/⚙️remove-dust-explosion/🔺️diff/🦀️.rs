//! ⚙ `remove-dust-explosion` diff: removes the addressed dust explosion row.

use crate::remove_dust_explosion::RemoveDustExplosion;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DustExplosionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveDustExplosion, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.dust_explosions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dust explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { dust_explosions: En1991DustExplosionDelta::removal(&base.dust_explosions, index), ..Default::default() })
}
