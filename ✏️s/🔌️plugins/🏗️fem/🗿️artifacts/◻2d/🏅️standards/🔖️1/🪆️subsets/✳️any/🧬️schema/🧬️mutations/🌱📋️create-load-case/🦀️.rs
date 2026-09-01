//! 🌱️ Fem2d mutation — `CreateLoadCase` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemLoad, FemLoadCase, element_id};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, delete_load_case};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemLoadCase`] into existence (empty or pre-seeded with one load — the
/// resolve-or-create gesture in `🎮️commands/🏋️add-nodal-load` builds this when no matching case exists yet).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-load-case")]
pub struct CreateLoadCase {
    pub load_case: FemLoadCase,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateLoadCase {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load-case", kind: "create-load-case", record: "CreatedLoadCase" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
