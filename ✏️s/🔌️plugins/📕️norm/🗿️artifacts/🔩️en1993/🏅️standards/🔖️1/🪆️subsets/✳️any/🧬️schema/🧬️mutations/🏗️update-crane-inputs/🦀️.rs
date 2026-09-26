//! `upsert-crane-runway` — upsert a `CraneRunway` by id into `crane_runways`.

use crate::{CraneRunway, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateCraneInputs {
    pub crane_runway: CraneRunway,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateCraneInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "craneRunway", kind: "update-crane-inputs", record: "UpdatedCraneRunway" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert crane runway {}", self.crane_runway.id),
            &format!("Kranbahn setzen {}", self.crane_runway.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.crane_runway.id.clone()]
    }
}
//#endregion 🔖️Payload
