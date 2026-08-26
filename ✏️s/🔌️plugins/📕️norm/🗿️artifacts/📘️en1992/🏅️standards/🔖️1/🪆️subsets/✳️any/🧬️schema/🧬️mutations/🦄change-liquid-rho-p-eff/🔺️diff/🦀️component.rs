//! 🔺️ `change-liquid-rho-p-eff` sparse diff construction — writes only `En1992Diff.liquid_rho_p_eff` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_rho_p_eff::mutation::ChangeLiquidRhoPEff;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidRhoPEff, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_liquid_rho_p_eff.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Liquid rho p eff must be a finite number.", Vec::<String>::new());
    }
    if base.liquid_rho_p_eff == payload.new_liquid_rho_p_eff {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Liquid rho p eff already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { liquid_rho_p_eff: Some(payload.new_liquid_rho_p_eff.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
