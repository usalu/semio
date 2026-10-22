//! 📎️ `change-annex` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: String,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeAnnex where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_annex::ChangeAnnex> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "annex",
        kind: "change-annex",
        record: "ChangeAnnex",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change national annex", "Nationalen Anhang ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-annex".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeAnnex where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_annex::ChangeAnnex> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_annex::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_annex::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
