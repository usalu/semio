//! 🔁️ Fem2d mutation — `ReplaceSection` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemSection};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSectionsDelta, Fem2dSectionsPatchEntry};
use crate::artifacts::fem2d::mutations::Fem2dMutation;
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

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceSection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "section", kind: "replace-section", record: "ReplacedSection" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
