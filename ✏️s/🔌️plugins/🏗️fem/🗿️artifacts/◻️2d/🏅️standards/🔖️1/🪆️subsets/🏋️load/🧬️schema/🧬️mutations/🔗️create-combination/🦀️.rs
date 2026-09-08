//! 🌱️ Fem2d mutation — `CreateCombination` payload + `MutationKind` impl.

use crate::{Fem2dSnapshot, FemCombination};
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemCombination`] into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-combination")]
pub struct CreateCombination {
    pub combination: FemCombination,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateCombination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "combination", kind: "create-combination", record: "CreatedCombination" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create combination \"{}\"", self.combination.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.combination.id.clone()]
    }
}
//#endregion 🔖️Mutation
