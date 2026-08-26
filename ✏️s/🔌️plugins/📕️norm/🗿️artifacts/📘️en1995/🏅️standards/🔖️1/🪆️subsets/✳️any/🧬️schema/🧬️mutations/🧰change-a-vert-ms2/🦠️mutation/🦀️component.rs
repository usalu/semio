//! 🔧 `change-a-vert-m-s2` payload — changes the En1995 document's `a_vert_m_s2` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAVertMS2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAVertMS2 {
    pub new_a_vert_m_s2: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAVertMS2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "a-vert-ms2", kind: "change-a-vert-ms2", record: "ChangedAVertMS2" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_a_vert_m_s2::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_a_vert_m_s2::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change a vert m s2 to {:?}", self.new_a_vert_m_s2)
    }
}
//#endregion 🔖️ChangeAVertMS2
