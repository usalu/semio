//! 🏭️ `change-manufacturer-file` — atomically updates the manufacturer file header facet (the
//! norm's `010` record fields are always authored together, never one-field-at-a-time).

use semio_s_artifact_norm_vdi3805_model::{ManufacturerFile, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeManufacturerFile {
    pub new_manufacturer_file: ManufacturerFile,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeManufacturerFile where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_manufacturer_file::ChangeManufacturerFile> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "manufacturer-file", kind: "change-manufacturer-file", record: "ChangedManufacturerFile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update manufacturer file header (manufacturer=\"{}\")", self.new_manufacturer_file.manufacturer), &format!("Herstellerdateikopf (Hersteller=\"{}\") aktualisieren", self.new_manufacturer_file.manufacturer))
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for ChangeManufacturerFile where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_manufacturer_file::ChangeManufacturerFile> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::change_manufacturer_file::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_manufacturer_file::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
