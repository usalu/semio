//! 📈️ `create-curve` — brings a new id-keyed characteristic curve into existence.


use crate::artifacts::vdi3805::{CharacteristicCurve, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::delete_curve;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurve {
    pub curve: CharacteristicCurve,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for CreateCurve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "curve", kind: "create-curve", record: "CreatedCurve" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create curve \"{}\"", self.curve.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.curve.id.clone()]
    }
}
//#endregion 🔖️Payload
