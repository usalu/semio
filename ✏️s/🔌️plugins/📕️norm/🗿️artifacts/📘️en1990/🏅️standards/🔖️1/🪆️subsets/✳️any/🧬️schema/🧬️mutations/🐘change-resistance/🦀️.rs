//! 🛡️ `change-resistance` — sets the EN 1990 document's design resistance `R_d` [kN], checked
//! against the combined design actions.


use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeResistance {
    pub new_resistance_kn: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeResistance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "resistance", kind: "change-resistance", record: "ChangedResistance" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change resistance to {} kN", self.new_resistance_kn)
    }
}
//#endregion 🔖️Payload
