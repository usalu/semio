//! `upsert-material` — upsert a `SteelMaterial` by id into `materials`.

use crate::{SteelMaterial, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateStainlessInputs {
    pub material: SteelMaterial,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateStainlessInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "material", kind: "update-stainless-inputs", record: "UpdatedMaterial" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert material {}", self.material.id),
            &format!("Werkstoff setzen {}", self.material.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }
}
//#endregion 🔖️Payload
