//! 🔥 `change-bed-joint-thickness-mm` payload — changes the En1996 document's `bed_joint_thickness_mm` (bed joint thickness [mm]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_bed_joint_thickness_mm::ChangeBedJointThicknessMm;

//#region 🔖️ChangeBedJointThicknessMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBedJointThicknessMm {
    pub new_bed_joint_thickness_mm: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeBedJointThicknessMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bed-joint-thickness-mm", kind: "change-bed-joint-thickness-mm", record: "ChangedBedJointThicknessMm" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change bed joint thickness [mm] to {}", self.new_bed_joint_thickness_mm)
    }
}
//#endregion 🔖️ChangeBedJointThicknessMm
