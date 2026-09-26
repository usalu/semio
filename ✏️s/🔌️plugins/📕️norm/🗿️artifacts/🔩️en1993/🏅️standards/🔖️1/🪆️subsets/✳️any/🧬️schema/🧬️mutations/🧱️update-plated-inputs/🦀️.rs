//! `upsert-plated-panel` — upsert a `PlatedPanel` by id into `plated_panels`.

use crate::{PlatedPanel, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePlatedInputs {
    pub plated_panel: PlatedPanel,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePlatedInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "platedPanel", kind: "update-plated-inputs", record: "UpdatedPlatedPanel" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert plated panel {}", self.plated_panel.id),
            &format!("Beulblech setzen {}", self.plated_panel.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.plated_panel.id.clone()]
    }
}
//#endregion 🔖️Payload
