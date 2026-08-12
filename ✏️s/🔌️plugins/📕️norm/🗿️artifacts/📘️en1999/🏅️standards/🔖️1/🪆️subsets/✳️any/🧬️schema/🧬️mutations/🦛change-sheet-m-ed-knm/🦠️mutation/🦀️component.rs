//! 🦛 `change-sheet-m-ed-knm` payload — changes the En1999 document's `sheet_m_ed_knm` (sheet design moment M_Ed [kNm]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSheetMEdKnm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSheetMEdKnm {
    pub new_sheet_m_ed_knm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSheetMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sheet-m-ed-knm", kind: "change-sheet-m-ed-knm", record: "ChangedSheetMEdKnm" };

    fn diff(&self, base: &En1999Snapshot) -> En1999Diff {
        crate::artifacts::en1999::mutations::change_sheet_m_ed_knm::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_sheet_m_ed_knm::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change sheet design moment M_Ed [kNm] to {}", self.new_sheet_m_ed_knm)
    }
}
//#endregion 🔖️ChangeSheetMEdKnm
