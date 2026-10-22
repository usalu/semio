//! 🏷️ `rename-catalogue` — renames the catalogue's identity field (metadata preferred name).

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RenameCatalogue {
    pub new_name: String,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for RenameCatalogue where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::rename_catalogue::mutation::RenameCatalogue> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "catalogue", kind: "rename-catalogue", record: "RenamedCatalogue" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename catalogue to \"{}\"", self.new_name), &format!("Katalog in \"{}\" umbenennen", self.new_name))
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for RenameCatalogue where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::rename_catalogue::mutation::RenameCatalogue> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::rename_catalogue::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rename_catalogue::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
