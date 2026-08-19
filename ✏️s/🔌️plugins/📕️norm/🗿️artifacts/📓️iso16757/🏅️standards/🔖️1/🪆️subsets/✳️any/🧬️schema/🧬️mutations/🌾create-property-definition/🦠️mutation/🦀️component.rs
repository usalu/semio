//! 🆕️ `create-property-definition` — brings a new id-keyed catalogue property definition into
//! existence.

use crate::artifacts::iso16757::{part_1::PropertyDefinition, Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePropertyDefinition {
    pub property_definition: PropertyDefinition,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for CreatePropertyDefinition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "property-definition", kind: "create-property-definition", record: "CreatedPropertyDefinition" };

    async fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create property definition \"{}\"", self.property_definition.names.preferred.text)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.property_definition.id.clone()]
    }
}
//#endregion 🔖️Payload
