//! `upsert-fatigue-detail` — upsert a `FatigueDetail` by id into `fatigue_details`.

use crate::{FatigueDetail, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateFatigueInputs {
    pub fatigue_detail: FatigueDetail,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateFatigueInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "fatigueDetail", kind: "update-fatigue-inputs", record: "UpdatedFatigueDetail" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert fatigue detail {}", self.fatigue_detail.id),
            &format!("Ermüdungsdetail setzen {}", self.fatigue_detail.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.fatigue_detail.id.clone()]
    }
}
//#endregion 🔖️Payload
