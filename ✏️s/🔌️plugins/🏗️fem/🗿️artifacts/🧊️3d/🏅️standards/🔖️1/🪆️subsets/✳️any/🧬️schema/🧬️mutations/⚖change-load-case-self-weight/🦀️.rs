//! ⚖️ Fem3d mutation — `ChangeLoadCaseSelfWeight` payload + `MutationKind` impl.

use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚖️ Toggles an existing load case's `self_weight` flag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-self-weight")]
pub struct ChangeLoadCaseSelfWeight {
    pub case_id: String,
    pub new_self_weight: bool,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ChangeLoadCaseSelfWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-self-weight", record: "ChangedLoadCaseSelfWeight" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set case \"{}\" self-weight to {}", self.case_id, self.new_self_weight)
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
