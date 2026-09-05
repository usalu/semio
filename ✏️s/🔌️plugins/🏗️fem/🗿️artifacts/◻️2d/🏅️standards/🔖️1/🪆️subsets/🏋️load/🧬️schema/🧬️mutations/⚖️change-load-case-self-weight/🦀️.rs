//! ⚖️ Fem2d mutation — `ChangeLoadCaseSelfWeight` payload + `MutationKind` impl.

use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ⚖️ Sets an existing load case's `self_weight` flag.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-self-weight")]
pub struct ChangeLoadCaseSelfWeight {
    pub case_id: String,
    pub new_self_weight: bool,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ChangeLoadCaseSelfWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-self-weight", record: "ChangedLoadCaseSelfWeight" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
