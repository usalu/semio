//! ➕️ Fem3d mutation — `AddLoad` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemLoad, load_id};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, remove_load};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕️ Attaches a [`FemLoad`] to an existing load case's `loads` member collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-load")]
pub struct AddLoad {
    pub case_id: String,
    #[dsl(statements)]
    pub load: Box<FemLoad>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for AddLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "load", kind: "add-load", record: "AddedLoad" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add load \"{}\" to case \"{}\"", load_id(&self.load), self.case_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
