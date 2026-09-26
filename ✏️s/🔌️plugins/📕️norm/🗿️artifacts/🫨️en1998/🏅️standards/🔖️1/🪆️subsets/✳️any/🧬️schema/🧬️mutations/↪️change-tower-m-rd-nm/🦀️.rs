//! ↪️ `change-tower-m-rd-nm` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTowerMRdNm {
    pub index: usize,
    pub new_m_rd_nm: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerMRdNm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "tower-m-rd-nm",
        kind: "change-tower-m-rd-nm",
        record: "ChangeTowerMRdNm",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-tower-m-rd-nm", "change-tower-m-rd-nm")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-tower-m-rd-nm".into()]
    }
}
