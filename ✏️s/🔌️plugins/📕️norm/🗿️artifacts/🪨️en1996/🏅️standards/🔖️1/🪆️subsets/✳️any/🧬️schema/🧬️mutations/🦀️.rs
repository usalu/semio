//! 🧬️ En1996 artifact — document mutation dispatch for masonry building subject.

use crate::{En1996Diff, En1996Snapshot};

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

impl En1996Mutation {
    /// 🧩 Build a mutation bundle that transforms `base` into `target`.

    pub fn from_snapshot(base: &En1996Snapshot, target: &En1996Snapshot) -> Vec<Self> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.masonry_class != target.masonry_class {
            out.push(En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class: target.masonry_class }));
        }
        if base.design_situation != target.design_situation {
            out.push(En1996Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: target.design_situation }));
        }
        if base.storeys != target.storeys {
            out.push(En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys: target.storeys }));
        }
        let mut bi = 0usize;
        let mut ti = 0usize;
        let mut working = base.walls.clone();
        while bi < working.len() || ti < target.walls.len() {
            if bi < working.len() && (ti >= target.walls.len() || working[bi].id != target.walls.get(ti).map(|w| w.id.as_str()).unwrap_or("")) {
                out.push(En1996Mutation::RemoveWall(remove_wall::RemoveWall { index: bi }));
                working.remove(bi);
                continue;
            }
            if ti < target.walls.len() && (bi >= working.len() || working[bi].id != target.walls[ti].id) {
                out.push(En1996Mutation::InsertWall(insert_wall::InsertWall { index: bi, wall: target.walls[ti].clone() }));
                working.insert(bi, target.walls[ti].clone());
                bi += 1; ti += 1;
                continue;
            }
            if bi < working.len() && ti < target.walls.len() {
                let w0 = &working[bi];
                let w1 = &target.walls[ti];
                let index = bi;
                macro_rules! ch {
                    ($cond:expr, $var:expr) => { if $cond { out.push($var); } };
                }
                ch!(w0.thickness_m != w1.thickness_m, En1996Mutation::ChangeWallThickness(change_wall_thickness::ChangeWallThickness { index, new_thickness_m: w1.thickness_m }));
                ch!(w0.height_m != w1.height_m, En1996Mutation::ChangeWallHeight(change_wall_height::ChangeWallHeight { index, new_height_m: w1.height_m }));
                ch!(w0.length_m != w1.length_m, En1996Mutation::ChangeWallLength(change_wall_length::ChangeWallLength { index, new_length_m: w1.length_m }));
                ch!(w0.wall_type != w1.wall_type, En1996Mutation::ChangeWallType(change_wall_type::ChangeWallType { index, new_wall_type: w1.wall_type }));
                ch!(w0.label_en != w1.label_en, En1996Mutation::ChangeWallLabelEn(change_wall_label_en::ChangeWallLabelEn { index, new_label_en: w1.label_en.clone() }));
                ch!(w0.label_de != w1.label_de, En1996Mutation::ChangeWallLabelDe(change_wall_label_de::ChangeWallLabelDe { index, new_label_de: w1.label_de.clone() }));
                ch!(w0.support_sides != w1.support_sides, En1996Mutation::ChangeSupportSides(change_support_sides::ChangeSupportSides { index, new_support_sides: w1.support_sides }));
                ch!(w0.slab_bearing_depth_m != w1.slab_bearing_depth_m, En1996Mutation::ChangeSlabBearingDepth(change_slab_bearing_depth::ChangeSlabBearingDepth { index, new_slab_bearing_depth_m: w1.slab_bearing_depth_m }));
                ch!(w0.eccentricity_top_m != w1.eccentricity_top_m, En1996Mutation::ChangeEccentricityTop(change_eccentricity_top::ChangeEccentricityTop { index, new_eccentricity_top_m: w1.eccentricity_top_m }));
                ch!(w0.eccentricity_bottom_m != w1.eccentricity_bottom_m, En1996Mutation::ChangeEccentricityBottom(change_eccentricity_bottom::ChangeEccentricityBottom { index, new_eccentricity_bottom_m: w1.eccentricity_bottom_m }));
                ch!(w0.unit_group != w1.unit_group, En1996Mutation::ChangeUnitGroup(change_unit_group::ChangeUnitGroup { index, new_unit_group: w1.unit_group }));
                ch!(w0.unit_material != w1.unit_material, En1996Mutation::ChangeUnitMaterial(change_unit_material::ChangeUnitMaterial { index, new_unit_material: w1.unit_material }));
                ch!(w0.f_b_pa != w1.f_b_pa, En1996Mutation::ChangeUnitFb(change_unit_fb::ChangeUnitFb { index, new_f_b_pa: w1.f_b_pa }));
                ch!(w0.unit_length_m != w1.unit_length_m, En1996Mutation::ChangeUnitLength(change_unit_length::ChangeUnitLength { index, new_unit_length_m: w1.unit_length_m }));
                ch!(w0.unit_width_m != w1.unit_width_m, En1996Mutation::ChangeUnitWidth(change_unit_width::ChangeUnitWidth { index, new_unit_width_m: w1.unit_width_m }));
                ch!(w0.unit_height_m != w1.unit_height_m, En1996Mutation::ChangeUnitHeight(change_unit_height::ChangeUnitHeight { index, new_unit_height_m: w1.unit_height_m }));
                ch!(w0.mortar_type != w1.mortar_type, En1996Mutation::ChangeMortarType(change_mortar_type::ChangeMortarType { index, new_mortar_type: w1.mortar_type }));
                ch!(w0.mortar_class != w1.mortar_class, En1996Mutation::ChangeMortarClass(change_mortar_class::ChangeMortarClass { index, new_mortar_class: w1.mortar_class }));
                ch!(w0.mortar_strength_pa != w1.mortar_strength_pa, En1996Mutation::ChangeFm(change_fm::ChangeFm { index, new_mortar_strength_pa: w1.mortar_strength_pa }));
                ch!(w0.bed_joint_thickness_m != w1.bed_joint_thickness_m, En1996Mutation::ChangeBedJointThickness(change_bed_joint_thickness::ChangeBedJointThickness { index, new_bed_joint_thickness_m: w1.bed_joint_thickness_m }));
                ch!(w0.reinforced != w1.reinforced, En1996Mutation::ChangeReinforced(change_reinforced::ChangeReinforced { index, new_reinforced: w1.reinforced }));
                ch!(w0.as_vertical_m2 != w1.as_vertical_m2, En1996Mutation::ChangeAsVertical(change_as_vertical::ChangeAsVertical { index, new_as_vertical_m2: w1.as_vertical_m2 }));
                ch!(w0.as_horizontal_m2 != w1.as_horizontal_m2, En1996Mutation::ChangeAsHorizontal(change_as_horizontal::ChangeAsHorizontal { index, new_as_horizontal_m2: w1.as_horizontal_m2 }));
                ch!(w0.f_yd_pa != w1.f_yd_pa, En1996Mutation::ChangeFYd(change_f_yd::ChangeFYd { index, new_f_yd_pa: w1.f_yd_pa }));
                ch!(w0.fire_rei_min != w1.fire_rei_min, En1996Mutation::ChangeFireRei(change_fire_rei::ChangeFireRei { index, new_fire_rei_min: w1.fire_rei_min }));
                ch!(w0.exposure != w1.exposure, En1996Mutation::ChangeExposure(change_exposure::ChangeExposure { index, new_exposure: w1.exposure }));
                ch!(w0.mu != w1.mu, En1996Mutation::ChangeMu(change_mu::ChangeMu { index, new_mu: w1.mu }));
                ch!(w0.density_kg_m3 != w1.density_kg_m3, En1996Mutation::ChangeDensity(change_density::ChangeDensity { index, new_density_kg_m3: w1.density_kg_m3 }));
                ch!(w0.phi_infinity != w1.phi_infinity, En1996Mutation::ChangePhiInfinity(change_phi_infinity::ChangePhiInfinity { index, new_phi_infinity: w1.phi_infinity }));
                ch!(w0.is_basement != w1.is_basement, En1996Mutation::ChangeIsBasement(change_is_basement::ChangeIsBasement { index, new_is_basement: w1.is_basement }));
                // openings / load cases — structural list sync (insert/remove + scalar fields)
                let mut oi = 0usize; let mut oj = 0usize;
                let mut opens = w0.openings.clone();
                while oi < opens.len() || oj < w1.openings.len() {
                    if oi < opens.len() && (oj >= w1.openings.len() || opens[oi].id != w1.openings.get(oj).map(|o| o.id.as_str()).unwrap_or("")) {
                        out.push(En1996Mutation::RemoveOpening(remove_opening::RemoveOpening { wall_index: index, index: oi }));
                        opens.remove(oi); continue;
                    }
                    if oj < w1.openings.len() && (oi >= opens.len() || opens[oi].id != w1.openings[oj].id) {
                        out.push(En1996Mutation::InsertOpening(insert_opening::InsertOpening { wall_index: index, index: oi, opening: w1.openings[oj].clone() }));
                        opens.insert(oi, w1.openings[oj].clone()); oi+=1; oj+=1; continue;
                    }
                    if oi < opens.len() && oj < w1.openings.len() {
                        let a=&opens[oi]; let b=&w1.openings[oj];
                        if a.width_m != b.width_m { out.push(En1996Mutation::ChangeOpeningWidth(change_opening_width::ChangeOpeningWidth { wall_index: index, index: oi, new_width_m: b.width_m })); }
                        if a.height_m != b.height_m { out.push(En1996Mutation::ChangeOpeningHeight(change_opening_height::ChangeOpeningHeight { wall_index: index, index: oi, new_height_m: b.height_m })); }
                        if a.sill_height_m != b.sill_height_m { out.push(En1996Mutation::ChangeOpeningSill(change_opening_sill::ChangeOpeningSill { wall_index: index, index: oi, new_sill_height_m: b.sill_height_m })); }
                        oi+=1; oj+=1;
                    }
                }
                let mut li = 0usize; let mut lj = 0usize;
                let mut lcs = w0.load_cases.clone();
                while li < lcs.len() || lj < w1.load_cases.len() {
                    if li < lcs.len() && (lj >= w1.load_cases.len() || lcs[li].id != w1.load_cases.get(lj).map(|c| c.id.as_str()).unwrap_or("")) {
                        out.push(En1996Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { wall_index: index, index: li }));
                        lcs.remove(li); continue;
                    }
                    if lj < w1.load_cases.len() && (li >= lcs.len() || lcs[li].id != w1.load_cases[lj].id) {
                        out.push(En1996Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { wall_index: index, index: li, load_case: w1.load_cases[lj].clone() }));
                        lcs.insert(li, w1.load_cases[lj].clone()); li+=1; lj+=1; continue;
                    }
                    if li < lcs.len() && lj < w1.load_cases.len() {
                        let a=&lcs[li]; let b=&w1.load_cases[lj];
                        if a.design_situation != b.design_situation { out.push(En1996Mutation::ChangeLoadCaseSituation(change_load_case_situation::ChangeLoadCaseSituation { wall_index: index, load_case_index: li, new_design_situation: b.design_situation.clone() })); }
                        if a.imposed_category != b.imposed_category { out.push(En1996Mutation::ChangeImposedCategory(change_imposed_category::ChangeImposedCategory { wall_index: index, index: li, new_imposed_category: b.imposed_category.clone() })); }
                        if a.g_k_slab_n != b.g_k_slab_n { out.push(En1996Mutation::ChangeGKSlab(change_gk_slab::ChangeGKSlab { wall_index: index, index: li, new_g_k_slab_n: b.g_k_slab_n })); }
                        if a.q_k_imposed_pa != b.q_k_imposed_pa { out.push(En1996Mutation::ChangeQKImposed(change_qk_imposed::ChangeQKImposed { wall_index: index, index: li, new_q_k_imposed_pa: b.q_k_imposed_pa })); }
                        if a.tributary_area_m2 != b.tributary_area_m2 { out.push(En1996Mutation::ChangeTributaryArea(change_tributary_area::ChangeTributaryArea { wall_index: index, index: li, new_tributary_area_m2: b.tributary_area_m2 })); }
                        if a.slab_span_m != b.slab_span_m { out.push(En1996Mutation::ChangeSlabSpan(change_slab_span::ChangeSlabSpan { wall_index: index, index: li, new_slab_span_m: b.slab_span_m })); }
                        if a.q_k_snow_pa != b.q_k_snow_pa { out.push(En1996Mutation::ChangeQKSnow(change_qk_snow::ChangeQKSnow { wall_index: index, index: li, new_q_k_snow_pa: b.q_k_snow_pa })); }
                        if a.q_p_wind_pa != b.q_p_wind_pa { out.push(En1996Mutation::ChangeQPWind(change_qp_wind::ChangeQPWind { wall_index: index, index: li, new_q_p_wind_pa: b.q_p_wind_pa })); }
                        if a.c_pe != b.c_pe { out.push(En1996Mutation::ChangeCPe(change_c_pe::ChangeCPe { wall_index: index, index: li, new_c_pe: b.c_pe })); }
                        if a.h_k_earth_n != b.h_k_earth_n { out.push(En1996Mutation::ChangeHKEarth(change_hk_earth::ChangeHKEarth { wall_index: index, index: li, new_h_k_earth_n: b.h_k_earth_n })); }
                        // concentrated
                        let mut ci=0usize; let mut cj=0usize; let mut conc=a.concentrated.clone();
                        while ci < conc.len() || cj < b.concentrated.len() {
                            if ci < conc.len() && (cj >= b.concentrated.len() || conc[ci].id != b.concentrated.get(cj).map(|c| c.id.as_str()).unwrap_or("")) {
                                out.push(En1996Mutation::RemoveConcentrated(remove_concentrated::RemoveConcentrated { wall_index: index, load_case_index: li, index: ci }));
                                conc.remove(ci); continue;
                            }
                            if cj < b.concentrated.len() && (ci >= conc.len() || conc[ci].id != b.concentrated[cj].id) {
                                out.push(En1996Mutation::InsertConcentrated(insert_concentrated::InsertConcentrated { wall_index: index, load_case_index: li, index: ci, load: b.concentrated[cj].clone() }));
                                conc.insert(ci, b.concentrated[cj].clone()); ci+=1; cj+=1; continue;
                            }
                            if ci < conc.len() && cj < b.concentrated.len() {
                                let x=&conc[ci]; let y=&b.concentrated[cj];
                                if x.force_n != y.force_n { out.push(En1996Mutation::ChangeConcentratedForce(change_concentrated_force::ChangeConcentratedForce { wall_index: index, load_case_index: li, index: ci, new_force_n: y.force_n })); }
                                if x.bearing_area_m2 != y.bearing_area_m2 { out.push(En1996Mutation::ChangeConcentratedBearingArea(change_concentrated_bearing_area::ChangeConcentratedBearingArea { wall_index: index, load_case_index: li, index: ci, new_bearing_area_m2: y.bearing_area_m2 })); }
                                if x.bearing_length_m != y.bearing_length_m { out.push(En1996Mutation::ChangeConcentratedBearingLength(change_concentrated_bearing_length::ChangeConcentratedBearingLength { wall_index: index, load_case_index: li, index: ci, new_bearing_length_m: y.bearing_length_m })); }
                                ci+=1; cj+=1;
                            }
                        }
                        li+=1; lj+=1;
                    }
                }
                bi += 1; ti += 1;
            }
        }
        out
    }

}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes one committed mutation JSON document into [`En1996Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1996_mutation_json(text: &str) -> Result<En1996Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_en1996_mutation(base: &En1996Snapshot, mutation: &En1996Mutation) -> Result<(En1996Snapshot, Vec<String>), String> {
    let raised = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_en1996_mutation(mutation: &En1996Mutation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge
