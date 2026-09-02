//! 🌾 `change-silo-bulk-density-kn-m3` — sets the En1991 silo bulk density scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSiloBulkDensityKnM3 {
    pub new_silo_bulk_density_kn_m3: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloBulkDensityKnM3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-bulk-density-kn-m3", kind: "change-silo-bulk-density-kn-m3", record: "ChangedSiloBulkDensityKnM3" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change silo bulk density to {:?}", self.new_silo_bulk_density_kn_m3)
    }
}
//#endregion 🔖️Payload
