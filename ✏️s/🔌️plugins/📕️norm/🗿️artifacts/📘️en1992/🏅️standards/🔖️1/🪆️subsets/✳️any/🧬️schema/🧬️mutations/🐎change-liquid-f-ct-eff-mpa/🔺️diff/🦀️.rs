//! 🔺️ `change-liquid-f-ct-eff-mpa` sparse diff construction — writes only `En1992Diff.liquid_f_ct_eff_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidFCtEffMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_liquid_f_ct_eff_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Liquid f ct eff mpa must be a finite number.", Vec::<String>::new());
    }
    if base.liquid_f_ct_eff_mpa == payload.new_liquid_f_ct_eff_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Liquid f ct eff mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { liquid_f_ct_eff_mpa: Some(payload.new_liquid_f_ct_eff_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
