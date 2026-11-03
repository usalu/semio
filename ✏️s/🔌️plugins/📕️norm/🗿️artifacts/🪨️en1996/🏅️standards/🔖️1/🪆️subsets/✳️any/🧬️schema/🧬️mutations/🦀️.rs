//! 🧬️ En1996 artifact — document mutation dispatch for masonry building subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_bearing_length;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_slab_span;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_length;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_height;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_thickness;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_eccentricity_bottom;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_eccentricity_top;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_phi_infinity;
use semio_s_artifact_norm_en1996_mut_loads::change_qk_snow;
use semio_s_artifact_norm_en1996_mut_concentrated::insert_concentrated;
use semio_s_artifact_norm_en1996_mut_loads::insert_load_case;
use semio_s_artifact_norm_en1996_mut_openings::insert_opening;
use semio_s_artifact_norm_en1996_mut_walls::insert_wall;
use semio_s_artifact_norm_en1996_mut_concentrated::remove_concentrated;
use semio_s_artifact_norm_en1996_mut_loads::remove_load_case;
use semio_s_artifact_norm_en1996_mut_openings::remove_opening;
use semio_s_artifact_norm_en1996_mut_walls::remove_wall;
use semio_s_artifact_norm_en1996_mut_project::change_annex;
use semio_s_artifact_norm_en1996_mut_project::change_reliability_class;
use semio_s_artifact_norm_en1996_mut_loads::change_qp_wind;
use semio_s_artifact_norm_en1996_mut_loads::change_design_situation;
use semio_s_artifact_norm_en1996_mut_loads::change_load_case_situation;
use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_force;
use semio_s_artifact_norm_en1996_mut_loads::change_gk_slab;
use semio_s_artifact_norm_en1996_mut_loads::change_qk_imposed;
use semio_s_artifact_norm_en1996_mut_project::change_is_basement;
use semio_s_artifact_norm_en1996_mut_project::change_storeys;
use semio_s_artifact_norm_en1996_mut_units::change_masonry_class;
use semio_s_artifact_norm_en1996_mut_project::change_imposed_category;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_label_de;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_label_en;
use semio_s_artifact_norm_en1996_mut_walls::change_exposure;
use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_bearing_area;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_slab_bearing_depth;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_tributary_area;
use semio_s_artifact_norm_en1996_mut_project::change_fire_rei;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_as_horizontal;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_as_vertical;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_f_yd;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_reinforced;
use semio_s_artifact_norm_en1996_mut_mortar::change_bed_joint_thickness;
use semio_s_artifact_norm_en1996_mut_mortar::change_fm;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_class;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_type;
use semio_s_artifact_norm_en1996_mut_loads::change_c_pe;
use semio_s_artifact_norm_en1996_mut_units::change_density;
use semio_s_artifact_norm_en1996_mut_walls::change_support_sides;
use semio_s_artifact_norm_en1996_mut_units::change_unit_fb;
use semio_s_artifact_norm_en1996_mut_units::change_unit_group;
use semio_s_artifact_norm_en1996_mut_units::change_unit_height;
use semio_s_artifact_norm_en1996_mut_units::change_unit_length;
use semio_s_artifact_norm_en1996_mut_units::change_unit_material;
use semio_s_artifact_norm_en1996_mut_units::change_unit_width;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_type;
use semio_s_artifact_norm_en1996_mut_mortar::change_mu;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_height;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_sill;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_width;
use semio_s_artifact_norm_en1996_mut_loads::change_hk_earth;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_pier_position;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_pier_width;
use semio_s_artifact_norm_en1996_mut_piers_lintels::insert_pier;
use semio_s_artifact_norm_en1996_mut_piers_lintels::remove_pier;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_lintel_position;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_lintel_span;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_lintel_depth;
use semio_s_artifact_norm_en1996_mut_piers_lintels::change_lintel_bearing_length;
use semio_s_artifact_norm_en1996_mut_piers_lintels::insert_lintel;
use semio_s_artifact_norm_en1996_mut_piers_lintels::remove_lintel;
use semio_s_artifact_norm_en1996_mut_ties::change_tie_kind;
use semio_s_artifact_norm_en1996_mut_ties::change_tie_position;
use semio_s_artifact_norm_en1996_mut_ties::change_tie_level;
use semio_s_artifact_norm_en1996_mut_ties::change_tie_capacity;
use semio_s_artifact_norm_en1996_mut_ties::insert_tie;
use semio_s_artifact_norm_en1996_mut_ties::remove_tie;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_force;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_area;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_strength;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_position;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_eccentricity;
use semio_s_artifact_norm_en1996_mut_prestress::change_prestress_loss;
use semio_s_artifact_norm_en1996_mut_prestress::insert_prestress;
use semio_s_artifact_norm_en1996_mut_prestress::remove_prestress;
use semio_s_artifact_norm_en1996_mut_confined::change_confined_position;
use semio_s_artifact_norm_en1996_mut_confined::change_confined_width;
use semio_s_artifact_norm_en1996_mut_confined::change_confined_depth;
use semio_s_artifact_norm_en1996_mut_confined::change_confined_steel;
use semio_s_artifact_norm_en1996_mut_confined::insert_confined;
use semio_s_artifact_norm_en1996_mut_confined::remove_confined;
use semio_s_artifact_norm_en1996_mut_flanges::change_flange_length;
use semio_s_artifact_norm_en1996_mut_flanges::change_flange_thickness;
use semio_s_artifact_norm_en1996_mut_flanges::change_flange_position;
use semio_s_artifact_norm_en1996_mut_flanges::insert_flange;
use semio_s_artifact_norm_en1996_mut_flanges::remove_flange;
use semio_s_artifact_norm_en1996_mut_bars::change_bar_kind;
use semio_s_artifact_norm_en1996_mut_bars::change_bar_diameter;
use semio_s_artifact_norm_en1996_mut_bars::change_bar_length;
use semio_s_artifact_norm_en1996_mut_bars::change_bar_spacing;
use semio_s_artifact_norm_en1996_mut_bars::insert_bar;
use semio_s_artifact_norm_en1996_mut_bars::remove_bar;
use semio_s_artifact_norm_en1996_mut_site::change_frost_class;
use semio_s_artifact_norm_en1996_mut_site::change_cover;
use semio_s_artifact_norm_en1996_mut_site::change_deflection_limit;
use semio_s_artifact_norm_en1996_mut_site::change_soil_height;
use semio_s_artifact_norm_en1996_mut_site::change_soil_density;
use semio_s_artifact_norm_en1996_mut_site::change_isotherm_depth;
use semio_s_artifact_norm_en1996_mut_plan::change_plan_x;
use semio_s_artifact_norm_en1996_mut_plan::change_plan_y;
use semio_s_artifact_norm_en1996_mut_plan::change_plan_angle;
use semio_s_artifact_norm_en1996_mut_plan::change_in_plane_shear;
use semio_s_artifact_norm_en1996_mut_plan::change_in_plane_moment;
use semio_s_artifact_norm_en1996_mut_positions::change_opening_position;
use semio_s_artifact_norm_en1996_mut_positions::change_concentrated_position;
use semio_s_artifact_norm_en1996_mut_positions::change_concentrated_distance_a1;
use semio_s_artifact_norm_en1996_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1996_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1996_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1996_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1996_mut_placements::insert_placement;
use semio_s_artifact_norm_en1996_mut_placements::remove_placement;
use semio_s_artifact_norm_en1996_mut_design_cases::insert_case;
use semio_s_artifact_norm_en1996_mut_design_cases::remove_case;
use semio_s_artifact_norm_en1996_mut_design_cases::change_case_number;
use semio_s_artifact_norm_en1996_mut_design_cases::change_case_text;
use semio_s_artifact_norm_en1996_mut_design_cases::change_case_label_en;
use semio_s_artifact_norm_en1996_mut_design_cases::change_case_label_de;
use semio_s_artifact_norm_en1996_mut_design_cases::change_case_entry_id;
use semio_s_artifact_norm_en1996_mut_bracing::change_bracing_system;
use semio_s_artifact_norm_en1996_mut_bracing::change_diaphragm_stiffness;
use semio_s_artifact_norm_en1996_mut_bracing::change_wall_web_spacing_m;
use semio_s_artifact_norm_en1996_mut_chases::change_chase_wall;
use semio_s_artifact_norm_en1996_mut_chases::change_chase_kind;
use semio_s_artifact_norm_en1996_mut_chases_geometry::change_chase_x;
use semio_s_artifact_norm_en1996_mut_chases_geometry::change_chase_y;
use semio_s_artifact_norm_en1996_mut_chases_geometry::change_chase_width;
use semio_s_artifact_norm_en1996_mut_chases_geometry::change_chase_length;
use semio_s_artifact_norm_en1996_mut_chases_geometry::change_chase_depth;
use semio_s_artifact_norm_en1996_mut_chases::insert_chase;
use semio_s_artifact_norm_en1996_mut_chases::remove_chase;
use semio_s_artifact_norm_en1996_mut_movement_joints::change_movement_joint_wall;
use semio_s_artifact_norm_en1996_mut_movement_joints::change_movement_joint_position;
use semio_s_artifact_norm_en1996_mut_movement_joints::insert_movement_joint;
use semio_s_artifact_norm_en1996_mut_movement_joints::remove_movement_joint;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1996Snapshot, diff = En1996Diff, schema = "s.norm.en1996", reference = En1996Snapshot, reference_diff = En1996Diff)]
pub enum En1996Mutation {
    ChangeConcentratedBearingLength(change_concentrated_bearing_length::ChangeConcentratedBearingLength),
    ChangeSlabSpan(change_slab_span::ChangeSlabSpan),
    ChangeWallLength(change_wall_length::ChangeWallLength),
    ChangeWallHeight(change_wall_height::ChangeWallHeight),
    ChangeWallThickness(change_wall_thickness::ChangeWallThickness),
    ChangeEccentricityBottom(change_eccentricity_bottom::ChangeEccentricityBottom),
    ChangeEccentricityTop(change_eccentricity_top::ChangeEccentricityTop),
    ChangePhiInfinity(change_phi_infinity::ChangePhiInfinity),
    ChangeQKSnow(change_qk_snow::ChangeQKSnow),
    InsertConcentrated(insert_concentrated::InsertConcentrated),
    InsertLoadCase(insert_load_case::InsertLoadCase),
    InsertOpening(insert_opening::InsertOpening),
    InsertWall(insert_wall::InsertWall),
    RemoveConcentrated(remove_concentrated::RemoveConcentrated),
    RemoveLoadCase(remove_load_case::RemoveLoadCase),
    RemoveOpening(remove_opening::RemoveOpening),
    RemoveWall(remove_wall::RemoveWall),
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    ChangeQPWind(change_qp_wind::ChangeQPWind),
    ChangeDesignSituation(change_design_situation::ChangeDesignSituation),
    ChangeLoadCaseSituation(change_load_case_situation::ChangeLoadCaseSituation),
    ChangeConcentratedForce(change_concentrated_force::ChangeConcentratedForce),
    ChangeGKSlab(change_gk_slab::ChangeGKSlab),
    ChangeQKImposed(change_qk_imposed::ChangeQKImposed),
    ChangeIsBasement(change_is_basement::ChangeIsBasement),
    ChangeStoreys(change_storeys::ChangeStoreys),
    ChangeMasonryClass(change_masonry_class::ChangeMasonryClass),
    ChangeImposedCategory(change_imposed_category::ChangeImposedCategory),
    ChangeWallLabelDe(change_wall_label_de::ChangeWallLabelDe),
    ChangeWallLabelEn(change_wall_label_en::ChangeWallLabelEn),
    ChangeExposure(change_exposure::ChangeExposure),
    ChangeConcentratedBearingArea(change_concentrated_bearing_area::ChangeConcentratedBearingArea),
    ChangeSlabBearingDepth(change_slab_bearing_depth::ChangeSlabBearingDepth),
    ChangeTributaryArea(change_tributary_area::ChangeTributaryArea),
    ChangeFireRei(change_fire_rei::ChangeFireRei),
    ChangeAsHorizontal(change_as_horizontal::ChangeAsHorizontal),
    ChangeAsVertical(change_as_vertical::ChangeAsVertical),
    ChangeFYd(change_f_yd::ChangeFYd),
    ChangeReinforced(change_reinforced::ChangeReinforced),
    ChangeBedJointThickness(change_bed_joint_thickness::ChangeBedJointThickness),
    ChangeFm(change_fm::ChangeFm),
    ChangeMortarClass(change_mortar_class::ChangeMortarClass),
    ChangeMortarType(change_mortar_type::ChangeMortarType),
    ChangeCPe(change_c_pe::ChangeCPe),
    ChangeDensity(change_density::ChangeDensity),
    ChangeSupportSides(change_support_sides::ChangeSupportSides),
    ChangeUnitFb(change_unit_fb::ChangeUnitFb),
    ChangeUnitGroup(change_unit_group::ChangeUnitGroup),
    ChangeUnitHeight(change_unit_height::ChangeUnitHeight),
    ChangeUnitLength(change_unit_length::ChangeUnitLength),
    ChangeUnitMaterial(change_unit_material::ChangeUnitMaterial),
    ChangeUnitWidth(change_unit_width::ChangeUnitWidth),
    ChangeWallType(change_wall_type::ChangeWallType),
    ChangeMu(change_mu::ChangeMu),
    ChangeOpeningHeight(change_opening_height::ChangeOpeningHeight),
    ChangeOpeningSill(change_opening_sill::ChangeOpeningSill),
    ChangeOpeningWidth(change_opening_width::ChangeOpeningWidth),
    ChangeHKEarth(change_hk_earth::ChangeHKEarth),
    ChangePierPosition(change_pier_position::ChangePierPosition),
    ChangePierWidth(change_pier_width::ChangePierWidth),
    InsertPier(insert_pier::InsertPier),
    RemovePier(remove_pier::RemovePier),
    ChangeLintelPosition(change_lintel_position::ChangeLintelPosition),
    ChangeLintelSpan(change_lintel_span::ChangeLintelSpan),
    ChangeLintelDepth(change_lintel_depth::ChangeLintelDepth),
    ChangeLintelBearingLength(change_lintel_bearing_length::ChangeLintelBearingLength),
    InsertLintel(insert_lintel::InsertLintel),
    RemoveLintel(remove_lintel::RemoveLintel),
    ChangeTieKind(change_tie_kind::ChangeTieKind),
    ChangeTiePosition(change_tie_position::ChangeTiePosition),
    ChangeTieLevel(change_tie_level::ChangeTieLevel),
    ChangeTieCapacity(change_tie_capacity::ChangeTieCapacity),
    InsertTie(insert_tie::InsertTie),
    RemoveTie(remove_tie::RemoveTie),
    ChangePrestressForce(change_prestress_force::ChangePrestressForce),
    ChangePrestressArea(change_prestress_area::ChangePrestressArea),
    ChangePrestressStrength(change_prestress_strength::ChangePrestressStrength),
    ChangePrestressPosition(change_prestress_position::ChangePrestressPosition),
    ChangePrestressEccentricity(change_prestress_eccentricity::ChangePrestressEccentricity),
    ChangePrestressLoss(change_prestress_loss::ChangePrestressLoss),
    InsertPrestress(insert_prestress::InsertPrestress),
    RemovePrestress(remove_prestress::RemovePrestress),
    ChangeConfinedPosition(change_confined_position::ChangeConfinedPosition),
    ChangeConfinedWidth(change_confined_width::ChangeConfinedWidth),
    ChangeConfinedDepth(change_confined_depth::ChangeConfinedDepth),
    ChangeConfinedSteel(change_confined_steel::ChangeConfinedSteel),
    InsertConfined(insert_confined::InsertConfined),
    RemoveConfined(remove_confined::RemoveConfined),
    ChangeFlangeLength(change_flange_length::ChangeFlangeLength),
    ChangeFlangeThickness(change_flange_thickness::ChangeFlangeThickness),
    ChangeFlangePosition(change_flange_position::ChangeFlangePosition),
    InsertFlange(insert_flange::InsertFlange),
    RemoveFlange(remove_flange::RemoveFlange),
    ChangeBarKind(change_bar_kind::ChangeBarKind),
    ChangeBarDiameter(change_bar_diameter::ChangeBarDiameter),
    ChangeBarLength(change_bar_length::ChangeBarLength),
    ChangeBarSpacing(change_bar_spacing::ChangeBarSpacing),
    InsertBar(insert_bar::InsertBar),
    RemoveBar(remove_bar::RemoveBar),
    ChangeFrostClass(change_frost_class::ChangeFrostClass),
    ChangeCover(change_cover::ChangeCover),
    ChangeDeflectionLimit(change_deflection_limit::ChangeDeflectionLimit),
    ChangeSoilHeight(change_soil_height::ChangeSoilHeight),
    ChangeSoilDensity(change_soil_density::ChangeSoilDensity),
    ChangeIsothermDepth(change_isotherm_depth::ChangeIsothermDepth),
    ChangePlanX(change_plan_x::ChangePlanX),
    ChangePlanY(change_plan_y::ChangePlanY),
    ChangePlanAngle(change_plan_angle::ChangePlanAngle),
    ChangeInPlaneShear(change_in_plane_shear::ChangeInPlaneShear),
    ChangeInPlaneMoment(change_in_plane_moment::ChangeInPlaneMoment),
    ChangeOpeningPosition(change_opening_position::ChangeOpeningPosition),
    ChangeConcentratedPosition(change_concentrated_position::ChangeConcentratedPosition),
    ChangeConcentratedDistanceA1(change_concentrated_distance_a1::ChangeConcentratedDistanceA1),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    InsertCase(insert_case::InsertCase),
    RemoveCase(remove_case::RemoveCase),
    ChangeCaseNumber(change_case_number::ChangeCaseNumber),
    ChangeCaseText(change_case_text::ChangeCaseText),
    ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn),
    ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe),
    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),
    ChangeBracingSystem(change_bracing_system::ChangeBracingSystem),
    ChangeDiaphragmStiffness(change_diaphragm_stiffness::ChangeDiaphragmStiffness),
    ChangeWallWebSpacingM(change_wall_web_spacing_m::ChangeWallWebSpacingM),
    ChangeChaseWall(change_chase_wall::ChangeChaseWall),
    ChangeChaseKind(change_chase_kind::ChangeChaseKind),
    ChangeChaseX(change_chase_x::ChangeChaseX),
    ChangeChaseY(change_chase_y::ChangeChaseY),
    ChangeChaseWidth(change_chase_width::ChangeChaseWidth),
    ChangeChaseLength(change_chase_length::ChangeChaseLength),
    ChangeChaseDepth(change_chase_depth::ChangeChaseDepth),
    InsertChase(insert_chase::InsertChase),
    RemoveChase(remove_chase::RemoveChase),
    ChangeMovementJointWall(change_movement_joint_wall::ChangeMovementJointWall),
    ChangeMovementJointPosition(change_movement_joint_position::ChangeMovementJointPosition),
    InsertMovementJoint(insert_movement_joint::InsertMovementJoint),
    RemoveMovementJoint(remove_movement_joint::RemoveMovementJoint),
}

pub const KINDS: &[&str] = &[
    "change-concentrated-bearing-length",
    "change-slab-span",
    "change-wall-length",
    "change-wall-height",
    "change-wall-thickness",
    "change-eccentricity-bottom",
    "change-eccentricity-top",
    "change-phi-infinity",
    "change-qk-snow",
    "insert-concentrated",
    "insert-load-case",
    "insert-opening",
    "insert-wall",
    "remove-concentrated",
    "remove-load-case",
    "remove-opening",
    "remove-wall",
    "change-annex",
    "change-reliability-class",
    "change-qp-wind",
    "change-design-situation",
    "change-load-case-situation",
    "change-concentrated-force",
    "change-gk-slab",
    "change-qk-imposed",
    "change-is-basement",
    "change-storeys",
    "change-masonry-class",
    "change-imposed-category",
    "change-wall-label-de",
    "change-wall-label-en",
    "change-exposure",
    "change-concentrated-bearing-area",
    "change-slab-bearing-depth",
    "change-tributary-area",
    "change-fire-rei",
    "change-as-horizontal",
    "change-as-vertical",
    "change-f-yd",
    "change-reinforced",
    "change-bed-joint-thickness",
    "change-fm",
    "change-mortar-class",
    "change-mortar-type",
    "change-c-pe",
    "change-density",
    "change-support-sides",
    "change-unit-fb",
    "change-unit-group",
    "change-unit-height",
    "change-unit-length",
    "change-unit-material",
    "change-unit-width",
    "change-wall-type",
    "change-mu",
    "change-opening-height",
    "change-opening-sill",
    "change-opening-width",
    "change-hk-earth",
    "change-pier-position",
    "change-pier-width",
    "insert-pier",
    "remove-pier",
    "change-lintel-position",
    "change-lintel-span",
    "change-lintel-depth",
    "change-lintel-bearing-length",
    "insert-lintel",
    "remove-lintel",
    "change-tie-kind",
    "change-tie-position",
    "change-tie-level",
    "change-tie-capacity",
    "insert-tie",
    "remove-tie",
    "change-prestress-force",
    "change-prestress-area",
    "change-prestress-strength",
    "change-prestress-position",
    "change-prestress-eccentricity",
    "change-prestress-loss",
    "insert-prestress",
    "remove-prestress",
    "change-confined-position",
    "change-confined-width",
    "change-confined-depth",
    "change-confined-steel",
    "insert-confined",
    "remove-confined",
    "change-flange-length",
    "change-flange-thickness",
    "change-flange-position",
    "insert-flange",
    "remove-flange",
    "change-bar-kind",
    "change-bar-diameter",
    "change-bar-length",
    "change-bar-spacing",
    "insert-bar",
    "remove-bar",
    "change-frost-class",
    "change-cover",
    "change-deflection-limit",
    "change-soil-height",
    "change-soil-density",
    "change-isotherm-depth",
    "change-plan-x",
    "change-plan-y",
    "change-plan-angle",
    "change-in-plane-shear",
    "change-in-plane-moment",
    "change-opening-position",
    "change-concentrated-position",
    "change-concentrated-distance-a1",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-bracing-system",
    "change-diaphragm-stiffness",
    "change-wall-web-spacing-m",
    "change-chase-wall",
    "change-chase-kind",
    "change-chase-x",
    "change-chase-y",
    "change-chase-width",
    "change-chase-length",
    "change-chase-depth",
    "insert-chase",
    "remove-chase",
    "change-movement-joint-wall",
    "change-movement-joint-position",
    "insert-movement-joint",
    "remove-movement-joint",
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1996_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1996_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1996_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1996_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1996Mutation;
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;

/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_en1996_mutation(base: &En1996Snapshot, mutation: &En1996Mutation) -> Result<(En1996Snapshot, Vec<String>), String> {
    let raised = <En1996Mutation as protocol::ColdMutation<En1996Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_en1996_mutation(mutation: &En1996Mutation, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok({
    <En1996Mutation as protocol::ColdMutation<En1996Snapshot>>::inverse(mutation, base)?

    })
}
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
