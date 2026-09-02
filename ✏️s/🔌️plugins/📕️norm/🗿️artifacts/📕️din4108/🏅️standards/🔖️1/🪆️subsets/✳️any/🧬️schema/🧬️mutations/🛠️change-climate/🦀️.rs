//! 🌦️ `change-climate` — sets the DIN 4108 `climate` scalar.


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use crate::document::ClimateZoneDe;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeClimate {
    pub new_climate: ClimateZoneDe,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeClimate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "climate", kind: "change-climate", record: "ChangedClimate" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change climate to {:?}", self.new_climate)
    }
}
//#endregion 🔖️Payload
