//! ➖️ Fem3d mutation — `RemoveLoad` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, load_id};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, add_load};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// ➖️ Detaches a load from an existing load case's `loads` member collection by id, capturing
/// nothing itself (the removed payload is recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-load")]
pub struct RemoveLoad {
    pub case_id: String,
    pub load_id: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for RemoveLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "load", kind: "remove-load", record: "RemovedLoad" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove load \"{}\" from case \"{}\"", self.load_id, self.case_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
