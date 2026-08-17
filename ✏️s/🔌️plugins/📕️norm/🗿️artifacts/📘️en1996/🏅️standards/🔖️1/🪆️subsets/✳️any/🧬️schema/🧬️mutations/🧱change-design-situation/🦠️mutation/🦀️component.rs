//! 🧱 `change-design-situation` payload — changes the En1996 document's `design_situation` (design situation).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDesignSituation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDesignSituation {
    pub new_design_situation: crate::document::DesignSituation,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeDesignSituation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "design-situation", kind: "change-design-situation", record: "ChangedDesignSituation" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_design_situation::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_design_situation::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design situation to {:?}", self.new_design_situation)
    }
}
//#endregion 🔖️ChangeDesignSituation
