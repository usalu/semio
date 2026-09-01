//! 🦉 `change-foundation-p-rd-kpa` payload — changes the En1998 document's `foundation_p_rd_kpa` (foundation bearing resistance p_Rd [kPa]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_foundation_p_rd_kpa::ChangeFoundationPRdKpa;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFoundationPRdKpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFoundationPRdKpa {
    pub new_foundation_p_rd_kpa: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeFoundationPRdKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "foundation-p-rd-kpa", kind: "change-foundation-p-rd-kpa", record: "ChangedFoundationPRdKpa" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change foundation bearing resistance p_Rd [kPa] to {}", self.new_foundation_p_rd_kpa)
    }
}
//#endregion 🔖️ChangeFoundationPRdKpa
