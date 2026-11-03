//! 🔩️ `change-beam-steel-designation` mutation leaf.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBeamSteelDesignation {
    pub id: String,
    pub new_designation: String,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangeBeamSteelDesignation where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_steel_designation::ChangeBeamSteelDesignation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "beam-steel-designation", kind: "change-beam-steel-designation", record: "ChangedBeamSteelDesignation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change steel section designation", "Profilbezeichnung des Stahlträgers ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for ChangeBeamSteelDesignation where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_steel_designation::ChangeBeamSteelDesignation> + From<crate::change_beam_steel_height_m::ChangeBeamSteelHeightM> + From<crate::change_beam_steel_width_m::ChangeBeamSteelWidthM> + From<crate::change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM> + From<crate::change_beam_steel_tf_m::ChangeBeamSteelTfM> + From<crate::change_beam_steel_area_m2::ChangeBeamSteelAreaM2> + From<crate::change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3> + From<crate::change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4> + From<crate::change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1994Snapshot>>::ReferenceDiff> {
        crate::change_beam_steel_designation::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_beam_steel_designation::inverse::inverse(self, base)
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
