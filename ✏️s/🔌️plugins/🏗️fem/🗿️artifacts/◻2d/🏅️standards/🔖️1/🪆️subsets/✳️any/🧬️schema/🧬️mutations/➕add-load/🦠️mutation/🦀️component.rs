//! ➕️ Fem2d mutation — `AddLoad` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{load_id, Fem2dSnapshot, FemLoad};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕️ Attaches a [`FemLoad`] to an existing load case's `loads` member collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-load")]
pub struct AddLoad {
    pub case_id: String,
    #[dsl(statements)]
    pub load: Box<FemLoad>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for AddLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "load", kind: "add-load", record: "AddedLoad" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
