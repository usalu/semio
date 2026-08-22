//! 🌱️ Fem3d mutation — `CreateSection` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemSection};
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

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "section", kind: "create-section", record: "CreatedSection" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
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
