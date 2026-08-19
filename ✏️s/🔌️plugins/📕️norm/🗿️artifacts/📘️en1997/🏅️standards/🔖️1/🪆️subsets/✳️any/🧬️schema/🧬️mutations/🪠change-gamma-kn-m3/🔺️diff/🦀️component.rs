//! 🔺️ `change-gamma-kn-m3` sparse diff construction — writes only `En1997Diff.gamma_kn_m3` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_gamma_kn_m3::mutation::ChangeGammaKnM3;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeGammaKnM3, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_gamma_kn_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Soil unit weight [kN/m3] must be a finite number, got {}.", payload.new_gamma_kn_m3), Vec::<String>::new());
    }
    if base.gamma_kn_m3 == payload.new_gamma_kn_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Soil unit weight [kN/m3] is already {}.", payload.new_gamma_kn_m3));
    }
    protocol::MutationOutcome::new(En1997Diff { gamma_kn_m3: Some(payload.new_gamma_kn_m3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
