//! 🌱️ Fem2d mutation — `CreateSection` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemSection};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, delete_section};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSection`] cross-section into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-section")]
pub struct CreateSection {
    pub section: FemSection,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "section", kind: "create-section", record: "CreatedSection" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create section \"{}\"", self.section.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.section.id.clone()]
    }
}
//#endregion 🔖️Mutation
