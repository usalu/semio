//! 🏢️ `change-storey-drift` mutation leaf: sets `drift` of one storey.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyDrift {
    pub id: String,
    pub new_drift: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeStoreyDrift where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_storey_drift::ChangeStoreyDrift> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-drift",
        kind: "change-storey-drift",
        record: "ChangedStoreyDrift",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Horizontal drift δ_H,Ed of the storey", "Geschoss: Horizontale Kopfverschiebung δ_H,Ed ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeStoreyDrift where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_storey_drift::ChangeStoreyDrift> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_storey_drift::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_storey_drift::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
