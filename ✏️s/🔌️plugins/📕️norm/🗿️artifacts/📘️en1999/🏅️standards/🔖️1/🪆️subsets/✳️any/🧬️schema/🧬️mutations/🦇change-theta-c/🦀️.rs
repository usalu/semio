//! 🦇 `change-theta-c` payload — changes the En1999 document's `theta_c` (fatigue detail category theta_C [MPa]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_theta_c::ChangeThetaC;

//#region 🔖️ChangeThetaC
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeThetaC {
    pub new_theta_c: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeThetaC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-c", kind: "change-theta-c", record: "ChangedThetaC" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fatigue detail category theta_C [MPa] to {}", self.new_theta_c)
    }
}
//#endregion 🔖️ChangeThetaC
