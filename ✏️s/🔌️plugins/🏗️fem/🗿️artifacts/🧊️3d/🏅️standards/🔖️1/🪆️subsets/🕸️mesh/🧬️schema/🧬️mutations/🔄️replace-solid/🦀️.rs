//! 🔁️ Fem3d mutation — `ReplaceSolid` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemSolid};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing meshed solid's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-solid")]
pub struct ReplaceSolid {
    pub id: String,
    pub new_solid: FemSolid,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceSolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "solid", kind: "replace-solid", record: "ReplacedSolid" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace solid \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
