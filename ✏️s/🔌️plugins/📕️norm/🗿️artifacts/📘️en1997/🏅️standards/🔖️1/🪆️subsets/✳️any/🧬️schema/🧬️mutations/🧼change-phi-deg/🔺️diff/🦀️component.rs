//! 🔺️ `change-phi-deg` sparse diff construction — writes only `En1997Diff.phi_deg` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_phi_deg::mutation::ChangePhiDeg;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePhiDeg, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_phi_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Friction angle phi [deg] must be a finite number, got {}.", payload.new_phi_deg), Vec::<String>::new());
    }
    if base.phi_deg == payload.new_phi_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Friction angle phi [deg] is already {}.", payload.new_phi_deg));
    }
    protocol::MutationOutcome::new(En1997Diff { phi_deg: Some(payload.new_phi_deg.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
