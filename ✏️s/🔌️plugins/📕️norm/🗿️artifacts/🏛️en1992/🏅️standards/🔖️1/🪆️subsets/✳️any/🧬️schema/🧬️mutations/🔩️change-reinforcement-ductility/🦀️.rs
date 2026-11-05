//! 🔩 `change-reinforcement-ductility` mutation leaf: sets `ductility` of one reinforcement grade.

use semio_s_artifact_norm_en1992_model::{DuctilityClass, En1992Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeReinforcementDuctility {
    pub grade_id: String,
    pub new_ductility: DuctilityClass,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for ChangeReinforcementDuctility where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_reinforcement_ductility::ChangeReinforcementDuctility> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "reinforcement-grade-ductility",
        kind: "change-reinforcement-ductility",
        record: "ChangedReinforcementDuctility",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change ductility of the reinforcement grade", "Betonstahlsorte: Duktilitätsklasse ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.grade_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for ChangeReinforcementDuctility where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_reinforcement_ductility::ChangeReinforcementDuctility> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::change_reinforcement_ductility::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_reinforcement_ductility::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
