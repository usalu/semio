//! 🌉 `change-span-m` — sets the En 1994 beam span [m] scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSpanM {
    pub new_span_m: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeSpanM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "span-m", kind: "change-span-m", record: "ChangedSpanM" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change span to {}", self.new_span_m)
    }
}
//#endregion 🔖️Payload
