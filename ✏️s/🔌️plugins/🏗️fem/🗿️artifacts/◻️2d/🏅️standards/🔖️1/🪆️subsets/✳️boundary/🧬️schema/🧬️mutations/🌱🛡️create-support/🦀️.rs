//! 🌱️ Fem2d mutation — `CreateSupport` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemSupport};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, delete_support};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSupport`] into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-support")]
pub struct CreateSupport {
    pub support: FemSupport,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateSupport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "support", kind: "create-support", record: "CreatedSupport" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
