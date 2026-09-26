//! `upsert-load-case` — upsert a `LoadCase` by id into `load_cases`.

use crate::{LoadCase, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateHssInputs {
    pub load_case: LoadCase,
 }

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateHssInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "loadCase", kind: "update-hss-inputs", record: "UpdatedLoadCase" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert load case {}", self.load_case.id),
            &format!("Lastfall setzen {}", self.load_case.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.load_case.id.clone()]
    }
}
//#endregion 🔖️Payload
