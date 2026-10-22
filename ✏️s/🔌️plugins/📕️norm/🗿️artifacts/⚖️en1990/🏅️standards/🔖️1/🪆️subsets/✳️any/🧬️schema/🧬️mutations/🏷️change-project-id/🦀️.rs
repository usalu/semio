//! 🧬️ `change-project-id` mutation for EN 1990.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeProjectId {
    pub new_project_id: String,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeProjectId where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_project_id::ChangeProjectId> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "project-id",
        kind: "change-project-id",
        record: "ChangedProjectId",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change project identifier", "Projektkennung ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeProjectId where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_project_id::ChangeProjectId> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1990Snapshot>>::ReferenceDiff> {
        crate::change_project_id::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_project_id::inverse::inverse(self, base)?
    
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
