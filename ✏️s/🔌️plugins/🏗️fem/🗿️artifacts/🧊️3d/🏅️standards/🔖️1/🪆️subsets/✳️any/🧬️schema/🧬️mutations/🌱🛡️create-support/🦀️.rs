//! 🌱️ Fem3d mutation — `CreateSupport` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemSupport};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, delete_support};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSupport`] support into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-support")]
pub struct CreateSupport {
    pub support: FemSupport,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "support", kind: "create-support", record: "CreatedSupport" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
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
