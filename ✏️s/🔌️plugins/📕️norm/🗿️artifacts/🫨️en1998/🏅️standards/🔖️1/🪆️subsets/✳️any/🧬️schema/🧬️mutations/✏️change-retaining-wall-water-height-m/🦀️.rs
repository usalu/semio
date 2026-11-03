//! ✏️ `change-retaining-wall-water-height-m` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRetainingWallWaterHeightM {
    pub index: usize,
    pub new_water_height_m: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeRetainingWallWaterHeightM where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "retaining-wall-water-height-m",
        kind: "change-retaining-wall-water-height-m",
        record: "ChangeRetainingWallWaterHeightM",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Height of the retained water above the wall base for the hydrostatic and hydrodynamic water pressure (0 = none)", "Höhe des angestauten Wassers über der Wandsohle für den hydrostatischen und hydrodynamischen Wasserdruck (0 = keines) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-retaining-wall-water-height-m".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeRetainingWallWaterHeightM where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_retaining_wall_water_height_m::ChangeRetainingWallWaterHeightM> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_retaining_wall_water_height_m::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_retaining_wall_water_height_m::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
