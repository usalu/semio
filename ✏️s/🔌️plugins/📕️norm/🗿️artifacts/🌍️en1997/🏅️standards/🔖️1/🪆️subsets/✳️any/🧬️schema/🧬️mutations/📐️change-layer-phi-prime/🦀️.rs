//! 🧹 `change-layer-phi-prime` payload.

use crate::diff::En1997Diff;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerPhiPrime {
    pub id: String,
    pub new_phi_prime_deg: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeLayerPhiPrime {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "layer-phi-prime", kind: "change-layer-phi-prime", record: "ChangedLayerPhiPrime",
    };
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-layer-phi-prime", "change-layer-phi-prime")
    }
}
