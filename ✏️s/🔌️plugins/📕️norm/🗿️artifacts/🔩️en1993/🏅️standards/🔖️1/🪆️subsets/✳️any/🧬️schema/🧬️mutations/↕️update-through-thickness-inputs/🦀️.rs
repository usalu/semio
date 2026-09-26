//! `upsert-section` — upsert a `SteelSection` by id into `sections`.

use crate::{SteelSection, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateThroughThicknessInputs {
    pub section: SteelSection,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateThroughThicknessInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "section", kind: "update-through-thickness-inputs", record: "UpdatedSection" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert section {}", self.section.id),
            &format!("Querschnitt setzen {}", self.section.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.section.id.clone()]
    }
}
//#endregion 🔖️Payload
