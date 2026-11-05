//! 📍️ `change-placement-yaw` mutation leaf: sets `yaw` of one placement.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePlacementYaw {
    pub id: String,
    pub new_yaw: f64,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangePlacementYaw where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_placement_yaw::ChangePlacementYaw> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "placement-yaw",
        kind: "change-placement-yaw",
        record: "ChangedPlacementYaw",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change yaw about Z of the placement", "Platzierung: Gierwinkel um Z ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for ChangePlacementYaw where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_placement_yaw::ChangePlacementYaw> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1994_diff::En1994Diff> {
        crate::change_placement_yaw::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_placement_yaw::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
