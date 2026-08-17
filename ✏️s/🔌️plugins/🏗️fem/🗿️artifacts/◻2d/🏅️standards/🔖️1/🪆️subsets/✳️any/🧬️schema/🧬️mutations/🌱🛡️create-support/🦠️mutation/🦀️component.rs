//! 🌱️ Fem2d mutation — `CreateSupport` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemSupport};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSupport`] into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-support")]
pub struct CreateSupport {
    pub support: FemSupport,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "support", kind: "create-support", record: "CreatedSupport" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create support \"{}\"", self.support.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.support.id.clone()]
    }
}
//#endregion 🔖️Mutation
