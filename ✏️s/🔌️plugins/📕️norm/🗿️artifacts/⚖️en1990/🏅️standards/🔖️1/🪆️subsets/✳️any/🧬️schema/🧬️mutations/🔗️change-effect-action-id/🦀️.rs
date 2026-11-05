//! 🔗 `change-effect-action-id` mutation leaf: sets `action_id` of one member effect.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeEffectActionId {
    pub index: usize,
    pub new_action_id: String,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeEffectActionId where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_effect_action_id::ChangeEffectActionId> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "effect-action-id",
        kind: "change-effect-action-id",
        record: "ChangedEffectActionId",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change action of the member effect", "Bauteilwirkung: Einwirkung ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeEffectActionId where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_effect_action_id::ChangeEffectActionId> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1990_diff::En1990Diff> {
        crate::change_effect_action_id::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_effect_action_id::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
