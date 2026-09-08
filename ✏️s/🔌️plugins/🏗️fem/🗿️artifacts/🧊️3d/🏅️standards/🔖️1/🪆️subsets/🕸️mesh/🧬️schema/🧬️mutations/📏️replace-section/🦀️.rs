//! 🔁️ Fem3d mutation — `ReplaceSection` payload + `MutationKind` impl.

use crate::{Fem3dSnapshot, FemSection};
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing cross-section's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-section")]
pub struct ReplaceSection {
    pub id: String,
    pub new_section: FemSection,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "section", kind: "replace-section", record: "ReplacedSection" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace section \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
