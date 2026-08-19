//! ⚖️ Fem3d mutation — `ChangeLoadCaseSelfWeight` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚖️ Toggles an existing load case's `self_weight` flag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-self-weight")]
pub struct ChangeLoadCaseSelfWeight {
    pub case_id: String,
    pub new_self_weight: bool,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ChangeLoadCaseSelfWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-self-weight", record: "ChangedLoadCaseSelfWeight" };

    async fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Set case \"{}\" self-weight to {}", self.case_id, self.new_self_weight)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
