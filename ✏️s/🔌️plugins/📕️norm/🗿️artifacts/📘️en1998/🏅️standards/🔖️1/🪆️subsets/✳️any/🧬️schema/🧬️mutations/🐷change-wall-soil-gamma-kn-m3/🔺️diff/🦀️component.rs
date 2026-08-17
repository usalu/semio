//! 🔺️ `change-wall-soil-gamma-kn-m3` sparse diff construction — writes only `En1998Diff.wall_soil_gamma_kn_m3` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_soil_gamma_kn_m3::mutation::ChangeWallSoilGammaKnM3;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallSoilGammaKnM3, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_wall_soil_gamma_kn_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Wall backfill unit weight [kN/m3] must be a finite number, got {}.", payload.new_wall_soil_gamma_kn_m3), Vec::<String>::new());
    }
    if base.wall_soil_gamma_kn_m3 == payload.new_wall_soil_gamma_kn_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Wall backfill unit weight [kN/m3] is already {}.", payload.new_wall_soil_gamma_kn_m3));
    }
    protocol::MutationOutcome::new(En1998Diff { wall_soil_gamma_kn_m3: Some(payload.new_wall_soil_gamma_kn_m3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
