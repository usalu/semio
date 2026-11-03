//! 🧵 `change-movement-joint-position` mutation leaf: sets `position_m` of one movement joint.

use semio_s_artifact_norm_en1996_model::En1996Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMovementJointPosition {
    pub id: String,
    pub new_position_m: f64,
}

impl<Op> protocol::MutationKind<En1996Snapshot, Op> for ChangeMovementJointPosition where Op: protocol::Mutation<En1996Snapshot, Diff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_movement_joint_position::ChangeMovementJointPosition> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "movement-joint-position-m",
        kind: "change-movement-joint-position",
        record: "ChangedMovementJointPosition",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change joint position x of the movement joint", "Dehnfuge: Fugenlage x ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1996Snapshot, Op> for ChangeMovementJointPosition where Op: protocol::ColdMutation<En1996Snapshot, ReferenceDiff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_movement_joint_position::ChangeMovementJointPosition> {
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1996_diff::En1996Diff> {
        crate::change_movement_joint_position::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_movement_joint_position::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
