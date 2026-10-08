//! 🧬️ En1996 artifact — document mutation dispatch for masonry building subject.

use crate::{En1996Diff, En1996Snapshot};

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::EDIT_RULES;

use super::change_concentrated_bearing_length;
use super::change_slab_span;
use super::change_wall_length;
use super::change_wall_height;
use super::change_wall_thickness;
use super::change_eccentricity_bottom;
use super::change_eccentricity_top;
use super::change_phi_infinity;
use super::change_qk_snow;
use super::insert_concentrated;
use super::insert_load_case;
use super::insert_opening;
use super::insert_wall;
use super::remove_concentrated;
use super::remove_load_case;
use super::remove_opening;
use super::remove_wall;
use super::change_annex;
use super::change_qp_wind;
use super::change_design_situation;
use super::change_load_case_situation;
use super::change_concentrated_force;
use super::change_gk_slab;
use super::change_qk_imposed;
use super::change_is_basement;
use super::change_storeys;
use super::change_masonry_class;
use super::change_imposed_category;
use super::change_wall_label_de;
use super::change_wall_label_en;
use super::change_exposure;
use super::change_concentrated_bearing_area;
use super::change_slab_bearing_depth;
use super::change_tributary_area;
use super::change_fire_rei;
use super::change_as_horizontal;
use super::change_as_vertical;
use super::change_f_yd;
use super::change_reinforced;
use super::change_bed_joint_thickness;
use super::change_fm;
use super::change_mortar_class;
use super::change_mortar_type;
use super::change_c_pe;
use super::change_density;
use super::change_support_sides;
use super::change_unit_fb;
use super::change_unit_group;
use super::change_unit_height;
use super::change_unit_length;
use super::change_unit_material;
use super::change_unit_width;
use super::change_wall_type;
use super::change_mu;
use super::change_opening_height;
use super::change_opening_sill;
use super::change_opening_width;
use super::change_hk_earth;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1996Snapshot, diff = En1996Diff, schema = "s.norm.en1996")]
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
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
pub(crate) mod fixture_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🌉️ExternalCodecBridge

/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_en1996_mutation(base: &En1996Snapshot, mutation: &En1996Mutation) -> Result<(En1996Snapshot, Vec<String>), String> {
    let raised = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_en1996_mutation(mutation: &En1996Mutation, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok({
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🌉️ExternalCodecBridge
