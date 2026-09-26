//! 🌬️ `change-has-mechanical-ventilation`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeHasMechanicalVentilation {
    pub new_has_mechanical_ventilation: bool,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeHasMechanicalVentilation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "has-mechanical-ventilation",
        kind: "change-has-mechanical-ventilation",
        record: "ChangedHasMechanicalVentilation",
    };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-has-mechanical-ventilation", "change-has-mechanical-ventilation")
    }
}
