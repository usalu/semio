//! ⚖️ Fem2d mutation — `ChangeLoadCaseSelfWeight` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚖️ Sets an existing load case's `self_weight` flag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-self-weight")]
pub struct ChangeLoadCaseSelfWeight {
    pub case_id: String,
    pub new_self_weight: bool,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ChangeLoadCaseSelfWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-self-weight", record: "ChangedLoadCaseSelfWeight" };

    async fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
