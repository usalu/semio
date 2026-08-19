//! 🔺️ `change-liquid-es-mpa` sparse diff construction — writes only `En1992Diff.liquid_e_s_mpa` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_e_s_mpa::mutation::ChangeLiquidESMpa;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeLiquidESMpa, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_liquid_e_s_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Liquid es mpa must be a finite number.", Vec::<String>::new());
    }
    if base.liquid_e_s_mpa == payload.new_liquid_e_s_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Liquid es mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { liquid_e_s_mpa: Some(payload.new_liquid_e_s_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
