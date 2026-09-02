//! 🛋️ `change-design-approach` payload — changes the En1997 document's `design_approach` (design approach).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_design_approach::ChangeDesignApproach;

//#region 🔖️ChangeDesignApproach
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
