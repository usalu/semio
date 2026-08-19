//! 🔺️ `change-weld-throat-mm` sparse diff construction — writes only `En1999Diff.weld_throat_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_weld_throat_mm::mutation::ChangeWeldThroatMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeWeldThroatMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_weld_throat_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld throat thickness [mm] must be a finite number, got {}.", payload.new_weld_throat_mm), Vec::<String>::new());
    }
    if base.weld_throat_mm == payload.new_weld_throat_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Weld throat thickness [mm] is already {}.", payload.new_weld_throat_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { weld_throat_mm: Some(payload.new_weld_throat_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
