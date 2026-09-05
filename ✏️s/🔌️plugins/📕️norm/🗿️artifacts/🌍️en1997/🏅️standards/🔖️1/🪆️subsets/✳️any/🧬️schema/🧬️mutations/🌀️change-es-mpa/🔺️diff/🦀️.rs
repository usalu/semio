//! 🔺️ `change-es-mpa` sparse diff construction — writes only `En1997Diff.e_s_mpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_e_s_mpa::ChangeESMpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeESMpa, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_e_s_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Soil modulus E_s [MPa] must be a finite number, got {}.", payload.new_e_s_mpa), Vec::<String>::new());
    }
    if base.e_s_mpa == payload.new_e_s_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Soil modulus E_s [MPa] is already {}.", payload.new_e_s_mpa));
    }
    protocol::MutationOutcome::new(En1997Diff { e_s_mpa: Some(payload.new_e_s_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
