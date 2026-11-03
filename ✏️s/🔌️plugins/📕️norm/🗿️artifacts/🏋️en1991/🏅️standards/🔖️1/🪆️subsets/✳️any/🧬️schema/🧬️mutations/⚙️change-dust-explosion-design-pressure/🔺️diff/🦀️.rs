//! ⚙ `change-dust-explosion-design-pressure` diff: patches `design_pressure` of the addressed dust explosion row.

use crate::change_dust_explosion_design_pressure::ChangeDustExplosionDesignPressure;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DustExplosionDelta, En1991DustExplosionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeDustExplosionDesignPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_design_pressure.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.dust_explosions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Dust explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.design_pressure == payload.new_design_pressure {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The dust explosion already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { dust_explosions: En1991DustExplosionDelta::modification(&payload.id, En1991DustExplosionPatch { design_pressure: Some(payload.new_design_pressure), ..Default::default() }), ..Default::default() })
}
