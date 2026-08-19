//! 🔺️ `change-sheet-m-ed-knm` sparse diff construction — writes only `En1999Diff.sheet_m_ed_knm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sheet_m_ed_knm::mutation::ChangeSheetMEdKnm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSheetMEdKnm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_sheet_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sheet design moment M_Ed [kNm] must be a finite number, got {}.", payload.new_sheet_m_ed_knm), Vec::<String>::new());
    }
    if base.sheet_m_ed_knm == payload.new_sheet_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sheet design moment M_Ed [kNm] is already {}.", payload.new_sheet_m_ed_knm));
    }
    protocol::MutationOutcome::new(En1999Diff { sheet_m_ed_knm: Some(payload.new_sheet_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
