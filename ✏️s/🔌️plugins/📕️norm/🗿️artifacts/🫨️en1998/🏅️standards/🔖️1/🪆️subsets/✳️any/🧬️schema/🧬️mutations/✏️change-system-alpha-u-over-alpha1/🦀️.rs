//! ✏️ `change-system-alpha-u-over-alpha1` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSystemAlphaUOverAlpha1 {
    pub building_id: String,
    pub id: String,
    pub new_alpha_u_over_alpha_1: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeSystemAlphaUOverAlpha1 where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "system-alpha-u-over-alpha-1",
        kind: "change-system-alpha-u-over-alpha1",
        record: "ChangeSystemAlphaUOverAlpha1",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change structural overstrength ratio αu/α1", "Tragwerksüberfestigkeitsverhältnis αu/α1 ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-system-alpha-u-over-alpha1".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeSystemAlphaUOverAlpha1 where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_system_alpha_u_over_alpha_1::ChangeSystemAlphaUOverAlpha1> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_system_alpha_u_over_alpha_1::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_system_alpha_u_over_alpha_1::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
