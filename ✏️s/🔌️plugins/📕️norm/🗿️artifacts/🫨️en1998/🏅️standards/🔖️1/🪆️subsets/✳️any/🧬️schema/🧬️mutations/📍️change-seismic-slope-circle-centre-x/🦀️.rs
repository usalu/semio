//! 📍️ `change-seismic-slope-circle-centre-x` mutation leaf: sets `circle-centre-x` of one seismic_slope.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSeismicSlopeCircleCentreX {
    pub id: String,
    pub new_circle_centre_x: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeSeismicSlopeCircleCentreX where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_seismic_slope_circle_centre_x::ChangeSeismicSlopeCircleCentreX> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "tie-beam-circle-centre-x",
        kind: "change-seismic-slope-circle-centre-x",
        record: "ChangedSeismicSlopeYaw",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change slope: Circle centre x", "Böschung: Kreismittelpunkt x ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeSeismicSlopeCircleCentreX where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_seismic_slope_circle_centre_x::ChangeSeismicSlopeCircleCentreX> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1998_diff::En1998Diff> {
        crate::change_seismic_slope_circle_centre_x::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_seismic_slope_circle_centre_x::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
