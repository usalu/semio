//! 🌱️ Fem3d mutation — `CreateLoadCase` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoadCase};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemLoadCase`] into existence (empty or pre-seeded with one load — the
/// resolve-or-create gesture in `🎮️commands/🏋️add-nodal-load` builds this when no matching case exists yet).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-load-case")]
pub struct CreateLoadCase {
    pub load_case: FemLoadCase,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateLoadCase {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load-case", kind: "create-load-case", record: "CreatedLoadCase" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create load case \"{}\"", self.load_case.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.load_case.id.clone()]
    }
}
//#endregion 🔖️Mutation
