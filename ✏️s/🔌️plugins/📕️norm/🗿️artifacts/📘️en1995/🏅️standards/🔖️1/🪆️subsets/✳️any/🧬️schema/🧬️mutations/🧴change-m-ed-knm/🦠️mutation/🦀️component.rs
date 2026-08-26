//! 🔧 `change-m-ed-knm` payload — changes the En1995 document's `m_ed_knm` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeMEdKnm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMEdKnm {
    pub new_m_ed_knm: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeMEdKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-ed-knm", kind: "change-m-ed-knm", record: "ChangedMEdKnm" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_m_ed_knm::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_m_ed_knm::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change m ed knm to {:?}", self.new_m_ed_knm)
    }
}
//#endregion 🔖️ChangeMEdKnm
