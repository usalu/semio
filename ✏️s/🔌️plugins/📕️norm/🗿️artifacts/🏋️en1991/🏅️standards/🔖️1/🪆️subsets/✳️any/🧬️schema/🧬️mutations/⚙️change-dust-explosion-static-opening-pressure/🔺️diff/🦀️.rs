//! ⚙ `change-dust-explosion-static-opening-pressure` diff: patches `static_opening_pressure` of the addressed dust explosion row.

use crate::change_dust_explosion_static_opening_pressure::ChangeDustExplosionStaticOpeningPressure;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DustExplosionDelta, En1991DustExplosionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeDustExplosionStaticOpeningPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_static_opening_pressure.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.dust_explosions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dust explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.static_opening_pressure == payload.new_static_opening_pressure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The dust explosion already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { dust_explosions: En1991DustExplosionDelta::modification(&payload.id, En1991DustExplosionPatch { static_opening_pressure: Some(payload.new_static_opening_pressure), ..Default::default() }), ..Default::default() })
}
