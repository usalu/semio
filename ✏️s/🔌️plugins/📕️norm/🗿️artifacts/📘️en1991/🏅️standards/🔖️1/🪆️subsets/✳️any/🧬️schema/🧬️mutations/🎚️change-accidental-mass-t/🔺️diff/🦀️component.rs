//! 🔺️ `change-accidental-mass-t` — sparse diff construction.

use super::mutation::ChangeAccidentalMassT;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAccidentalMassT, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_accidental_mass_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Accidental mass t must be a finite number.", Vec::<String>::new());
    }
    if base.accidental_mass_t == payload.new_accidental_mass_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Accidental mass t already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { accidental_mass_t: Some(payload.new_accidental_mass_t.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
