//! 🔺️ `change-seismic-action` — sparse diff construction; writes only
//! `En1990Diff.seismic_a_ed_kn`.

use super::ChangeSeismicAction;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSeismicAction, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_seismic_a_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Seismic action A_Ed must be a finite number.", Vec::<String>::new());
    }
    if base.seismic_a_ed_kn == payload.new_seismic_a_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Seismic action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { seismic_a_ed_kn: Some(payload.new_seismic_a_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
