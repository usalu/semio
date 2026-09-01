//! 🗑️ Fem2d mutation — `DeleteSupport` payload + `MutationKind` impl.

use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, create_support};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🗑️ Removes an existing support by id, capturing nothing itself (the removed payload is recovered
/// from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-support")]
pub struct DeleteSupport {
    pub id: String,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for DeleteSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "support", kind: "delete-support", record: "DeletedSupport" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete support \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
