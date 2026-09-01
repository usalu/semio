//! 🐐 `change-weld-throat-mm` payload — changes the En1999 document's `weld_throat_mm` (weld throat thickness [mm]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_weld_throat_mm::ChangeWeldThroatMm;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWeldThroatMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWeldThroatMm {
    pub new_weld_throat_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeWeldThroatMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "weld-throat-mm", kind: "change-weld-throat-mm", record: "ChangedWeldThroatMm" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change weld throat thickness [mm] to {}", self.new_weld_throat_mm)
    }
}
//#endregion 🔖️ChangeWeldThroatMm
