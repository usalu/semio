//! `upsert-tension-component` — upsert a `TensionComponent` by id into `tension_components`.

use crate::{TensionComponent, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateTensionComponentInputs {
    pub tension_component: TensionComponent,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateTensionComponentInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "tensionComponent", kind: "update-tension-component-inputs", record: "UpdatedTensionComponent" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert tension component {}", self.tension_component.id),
            &format!("Zugglied setzen {}", self.tension_component.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.tension_component.id.clone()]
    }
}
//#endregion 🔖️Payload
