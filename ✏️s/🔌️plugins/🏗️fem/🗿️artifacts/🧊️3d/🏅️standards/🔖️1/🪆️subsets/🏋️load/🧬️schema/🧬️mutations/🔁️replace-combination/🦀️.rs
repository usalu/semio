//! 🔁️ Fem3d mutation — `ReplaceCombination` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemCombination};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing load combination's payload — the only way this vocabulary
/// re-weights, re-terms or renames a combination without deleting and recreating it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-combination")]
pub struct ReplaceCombination {
    pub id: String,
    pub new_combination: FemCombination,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceCombination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "combination", kind: "replace-combination", record: "ReplacedCombination" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Replace combination \"{}\"", self.id), &format!("Kombination \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
