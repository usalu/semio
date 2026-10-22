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
pub use crate::edit_rules::EDIT_RULES;

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

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
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
