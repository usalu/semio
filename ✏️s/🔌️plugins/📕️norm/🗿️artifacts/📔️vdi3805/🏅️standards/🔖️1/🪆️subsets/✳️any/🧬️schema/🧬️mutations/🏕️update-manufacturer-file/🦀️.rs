//! 🏭️ `update-manufacturer-file` — atomically updates the manufacturer file header facet (the
//! norm's `010` record fields are always authored together, never one-field-at-a-time).


use crate::artifacts::vdi3805::{ManufacturerFile, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateManufacturerFile {
    pub new_manufacturer_file: ManufacturerFile,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for UpdateManufacturerFile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "manufacturer-file", kind: "update-manufacturer-file", record: "UpdatedManufacturerFile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update manufacturer file header (manufacturer=\"{}\")", self.new_manufacturer_file.manufacturer)
    }
}
//#endregion 🔖️Payload
