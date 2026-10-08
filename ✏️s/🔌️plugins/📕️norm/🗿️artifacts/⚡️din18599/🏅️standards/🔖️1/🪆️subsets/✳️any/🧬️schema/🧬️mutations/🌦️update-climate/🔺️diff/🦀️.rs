//! 🔺️ `update-climate` sparse diff construction — sets the parent-owned climate and re-mints the derived
//! `climateTable` handle from it (design §20.15 model (a); no child content is read).

use crate::mutations::update_climate::UpdateClimate;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599ClimatePatch};

//#region 🔖️Diff

pub fn diff(payload: &UpdateClimate, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if payload.new_climate.theta_e_c.iter().any(|v| !v.is_finite()) || payload.new_climate.g_h_w_m2.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Monthly climate values must be finite, and irradiance must be non-negative.", Vec::<String>::new());
    }
    if base.climate == payload.new_climate {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Climate profile is already this value.");
    }
    let (old, new) = (&base.climate, &payload.new_climate);
    protocol::MutationOutcome::new(Din18599Diff {
        climate: Some(Din18599ClimatePatch {
            theta_e_c: (old.theta_e_c != new.theta_e_c).then(|| new.theta_e_c.clone()),
            g_h_w_m2: (old.g_h_w_m2 != new.g_h_w_m2).then(|| new.g_h_w_m2.clone()),
        }),
        climate_table: Some(crate::din18599_climate_table_child(&payload.new_climate)),
        ..Default::default()
    })
}
