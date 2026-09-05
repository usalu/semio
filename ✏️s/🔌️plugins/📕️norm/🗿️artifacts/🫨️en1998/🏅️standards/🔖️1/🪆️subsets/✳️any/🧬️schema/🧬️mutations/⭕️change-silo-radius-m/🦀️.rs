//! 🐢 `change-silo-radius-m` payload — changes the En1998 document's `silo_radius_m` (silo radius [m]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
//#region 🔖️ChangeSiloRadiusM
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSiloRadiusM {
    pub new_silo_radius_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSiloRadiusM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-radius-m", kind: "change-silo-radius-m", record: "ChangedSiloRadiusM" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change silo radius [m] to {}", self.new_silo_radius_m)
    }
}
//#endregion 🔖️ChangeSiloRadiusM
