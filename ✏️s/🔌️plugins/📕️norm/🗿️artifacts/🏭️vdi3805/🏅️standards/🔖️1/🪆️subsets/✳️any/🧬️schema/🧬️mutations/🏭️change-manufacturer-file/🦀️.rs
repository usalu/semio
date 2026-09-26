//! 🏭️ `change-manufacturer-file` — atomically updates the manufacturer file header facet (the
//! norm's `010` record fields are always authored together, never one-field-at-a-time).

use crate::{ManufacturerFile, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeManufacturerFile {
    pub new_manufacturer_file: ManufacturerFile,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeManufacturerFile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "manufacturer-file", kind: "change-manufacturer-file", record: "ChangedManufacturerFile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Update manufacturer file header (manufacturer=\"{}\")", self.new_manufacturer_file.manufacturer), &format!("Herstellerdateikopf (Hersteller=\"{}\") aktualisieren", self.new_manufacturer_file.manufacturer))
    }
}
//#endregion 🔖️Payload
