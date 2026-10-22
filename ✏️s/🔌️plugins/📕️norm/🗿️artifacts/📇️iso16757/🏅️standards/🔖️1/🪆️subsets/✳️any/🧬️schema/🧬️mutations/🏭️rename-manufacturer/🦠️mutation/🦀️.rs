//! 🏭️ `rename-manufacturer` — renames the catalogue's manufacturer identity field.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RenameManufacturer {
    pub new_name: String,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for RenameManufacturer where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::rename_manufacturer::mutation::RenameManufacturer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "manufacturer", kind: "rename-manufacturer", record: "RenamedManufacturer" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename manufacturer to \"{}\"", self.new_name), &format!("Hersteller in \"{}\" umbenennen", self.new_name))
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for RenameManufacturer where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::rename_manufacturer::mutation::RenameManufacturer> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::rename_manufacturer::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_manufacturer::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
