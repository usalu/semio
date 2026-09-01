//! 🛋️ `change-design-approach` payload — changes the En1997 document's `design_approach` (design approach).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_design_approach::ChangeDesignApproach;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDesignApproach
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDesignApproach {
    pub new_design_approach: String,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeDesignApproach {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "design-approach", kind: "change-design-approach", record: "ChangedDesignApproach" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design approach to \"{}\"", self.new_design_approach)
    }
}
//#endregion 🔖️ChangeDesignApproach
