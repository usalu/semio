//! 🐐 `change-weld-throat-mm` payload — changes the En1999 document's `weld_throat_mm` (weld throat thickness [mm]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWeldThroatMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeWeldThroatMm {
    pub new_weld_throat_mm: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeWeldThroatMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "weld-throat-mm", kind: "change-weld-throat-mm", record: "ChangedWeldThroatMm" };

    async fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::artifacts::en1999::mutations::change_weld_throat_mm::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_weld_throat_mm::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change weld throat thickness [mm] to {}", self.new_weld_throat_mm)
    }
}
//#endregion 🔖️ChangeWeldThroatMm
