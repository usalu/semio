//! 💪️ `change-system-base-shear-resistance-n` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSystemBaseShearResistanceN {
    pub building_index: usize,
    pub system_index: usize,
    pub new_base_shear_resistance_n: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSystemBaseShearResistanceN {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "system-base-shear-resistance-n",
        kind: "change-system-base-shear-resistance-n",
        record: "ChangeSystemBaseShearResistanceN",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-system-base-shear-resistance-n", "change-system-base-shear-resistance-n")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-system-base-shear-resistance-n".into()]
    }
}
