//! 🏷️ Fem3d mutation — `ChangeLoadCaseName` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::Fem3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🏷️ Sets an existing load case's human-readable `name`. The `id` is the identity every
/// combination term resolves through and is untouched, so a rename orphans nothing.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-name")]
pub struct ChangeLoadCaseName {
    pub case_id: String,
    pub new_name: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ChangeLoadCaseName {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-name", record: "ChangedLoadCaseName" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename case \"{}\" to \"{}\"", self.case_id, self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
