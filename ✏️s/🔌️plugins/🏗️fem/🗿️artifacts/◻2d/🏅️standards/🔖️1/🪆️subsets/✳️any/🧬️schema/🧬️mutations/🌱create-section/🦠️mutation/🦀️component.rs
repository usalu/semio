//! 🌱️ Fem2d mutation — `CreateSection` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemSection};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSection`] cross-section into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-section")]
pub struct CreateSection {
    pub section: FemSection,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "section", kind: "create-section", record: "CreatedSection" };

    async fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create section \"{}\"", self.section.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.section.id.clone()]
    }
}
//#endregion 🔖️Mutation
