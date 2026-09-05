//! 🔺️ `change-it-mm4` sparse diff construction — writes only `En1999Diff.i_t_mm4` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_i_t_mm4::ChangeITMm4;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeITMm4, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_i_t_mm4.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Torsion constant I_t [mm4] must be a finite number, got {}.", payload.new_i_t_mm4), Vec::<String>::new());
    }
    if base.i_t_mm4 == payload.new_i_t_mm4 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Torsion constant I_t [mm4] is already {}.", payload.new_i_t_mm4));
    }
    protocol::MutationOutcome::new(En1999Diff { i_t_mm4: Some(payload.new_i_t_mm4.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
