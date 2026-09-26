//! `upsert-joint` — upsert a `SteelJoint` by id into `joints`.

use crate::{SteelJoint, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateBoltInputs {
    pub joint: SteelJoint,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateBoltInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "joint", kind: "update-bolt-inputs", record: "UpdatedJoint" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert joint {}", self.joint.id),
            &format!("Anschluss setzen {}", self.joint.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.joint.id.clone()]
    }
}
//#endregion 🔖️Payload
