//! `upsert-pile` — upsert a `SteelPile` by id into `piles`.

use crate::{SteelPile, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePileInputs {
    pub pile: SteelPile,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePileInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "pile", kind: "update-pile-inputs", record: "UpdatedPile" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert pile {}", self.pile.id),
            &format!("Pfahl setzen {}", self.pile.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.pile.id.clone()]
    }
}
//#endregion 🔖️Payload
