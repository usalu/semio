//! 🧬️ En1998 artifact — closed semantic mutation dispatch enum.

use crate::diff::En1998Diff;
use crate::En1998Snapshot;

use super::change_annex;
use super::update_site;
use super::insert_building;
use super::remove_building;
use super::change_system_v_rd_n;
use super::change_storey_permanent_gk_n;
use super::change_storey_stiffness_x;
use super::change_storey_drift_xm;
use super::change_building_plan_regular;
use super::change_elevation_regular;
use super::change_member_detailing;
use super::change_masonry_wall_ratio;
use super::insert_bridge;
use super::change_bridge_v_rd_n;
use super::insert_assessment;
use super::change_assessment_rkn;
use super::insert_silo;
use super::insert_tank;
use super::insert_foundation;
use super::insert_retaining_wall;
use super::insert_tower;
use super::change_tower_m_rd_nm;
use super::remove_bridge;
use super::remove_assessment;
use super::remove_silo;
use super::remove_tank;
use super::remove_foundation;
use super::remove_retaining_wall;
use super::remove_tower;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1998Snapshot, diff = En1998Diff, schema = "s.norm.en1998")]
pub enum En1998Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    UpdateSite(update_site::UpdateSite),
    InsertBuilding(insert_building::InsertBuilding),
    RemoveBuilding(remove_building::RemoveBuilding),
    ChangeSystemVRdN(change_system_v_rd_n::ChangeSystemVRdN),
    ChangeStoreyPermanentGkN(change_storey_permanent_gk_n::ChangeStoreyPermanentGkN),
    ChangeStoreyStiffnessX(change_storey_stiffness_x::ChangeStoreyStiffnessX),
    ChangeStoreyDriftXM(change_storey_drift_xm::ChangeStoreyDriftXM),
    ChangeBuildingPlanRegular(change_building_plan_regular::ChangeBuildingPlanRegular),
    ChangeElevationRegular(change_elevation_regular::ChangeElevationRegular),
    ChangeMemberDetailing(change_member_detailing::ChangeMemberDetailing),
    ChangeMasonryWallRatio(change_masonry_wall_ratio::ChangeMasonryWallRatio),
    InsertBridge(insert_bridge::InsertBridge),
    ChangeBridgeVRdN(change_bridge_v_rd_n::ChangeBridgeVRdN),
    InsertAssessment(insert_assessment::InsertAssessment),
    ChangeAssessmentRKN(change_assessment_rkn::ChangeAssessmentRKN),
    InsertSilo(insert_silo::InsertSilo),
    InsertTank(insert_tank::InsertTank),
    InsertFoundation(insert_foundation::InsertFoundation),
    InsertRetainingWall(insert_retaining_wall::InsertRetainingWall),
    InsertTower(insert_tower::InsertTower),
    ChangeTowerMRdNm(change_tower_m_rd_nm::ChangeTowerMRdNm),
    RemoveBridge(remove_bridge::RemoveBridge),
    RemoveAssessment(remove_assessment::RemoveAssessment),
    RemoveSilo(remove_silo::RemoveSilo),
    RemoveTank(remove_tank::RemoveTank),
    RemoveFoundation(remove_foundation::RemoveFoundation),
    RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall),
    RemoveTower(remove_tower::RemoveTower),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "update-site",
    "insert-building",
    "remove-building",
    "change-system-v-rd-n",
    "change-storey-permanent-gk-n",
    "change-storey-stiffness-x",
    "change-storey-drift-xm",
    "change-building-plan-regular",
    "change-elevation-regular",
    "change-member-detailing",
    "change-masonry-wall-ratio",
    "insert-bridge",
    "change-bridge-v-rd-n",
    "insert-assessment",
    "change-assessment-rkn",
    "insert-silo",
    "insert-tank",
    "insert-foundation",
    "insert-retaining-wall",
    "insert-tower",
    "change-tower-m-rd-nm",
    "remove-bridge",
    "remove-assessment",
    "remove-silo",
    "remove-tank",
    "remove-foundation",
    "remove-retaining-wall",
    "remove-tower",
];

impl En1998Mutation {
    /// 🔀 Decompose a snapshot edit into the closed semantic mutation vocabulary.
    pub fn from_snapshot(base: &En1998Snapshot, target: &En1998Snapshot) -> Vec<En1998Mutation> {
        let mut mutations = Vec::new();
        if base.annex != target.annex {
            mutations.push(En1998Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex.clone() }));
        }
        if base.site != target.site {
            mutations.push(En1998Mutation::UpdateSite(update_site::UpdateSite { site: target.site.clone() }));
        }
        if base.buildings != target.buildings {
            for index in (0..base.buildings.len()).rev() {
                mutations.push(En1998Mutation::RemoveBuilding(remove_building::RemoveBuilding { index }));
            }
            for (index, building) in target.buildings.iter().enumerate() {
                mutations.push(En1998Mutation::InsertBuilding(insert_building::InsertBuilding { index, building: building.clone() }));
            }
        } else {
            for (bi, (bb, tb)) in base.buildings.iter().zip(target.buildings.iter()).enumerate() {
                if bb.plan_regular != tb.plan_regular {
                    mutations.push(En1998Mutation::ChangeBuildingPlanRegular(change_building_plan_regular::ChangeBuildingPlanRegular {
                        building_index: bi,
                        new_plan_regular: tb.plan_regular,
                    }));
                }
                if bb.elevation_regular != tb.elevation_regular {
                    mutations.push(En1998Mutation::ChangeElevationRegular(change_elevation_regular::ChangeElevationRegular {
                        building_index: bi,
                        new_elevation_regular: tb.elevation_regular,
                    }));
                }
                if bb.masonry_wall_area_ratio.to_bits() != tb.masonry_wall_area_ratio.to_bits() {
                    mutations.push(En1998Mutation::ChangeMasonryWallRatio(change_masonry_wall_ratio::ChangeMasonryWallRatio {
                        building_index: bi,
                        new_masonry_wall_area_ratio: tb.masonry_wall_area_ratio,
                    }));
                }
                for (si, (bs, ts)) in bb.systems.iter().zip(tb.systems.iter()).enumerate() {
                    if bs.base_shear_resistance_n.to_bits() != ts.base_shear_resistance_n.to_bits() {
                        mutations.push(En1998Mutation::ChangeSystemVRdN(change_system_v_rd_n::ChangeSystemVRdN {
                            building_index: bi,
                            system_index: si,
                            new_base_shear_resistance_n: ts.base_shear_resistance_n,
                        }));
                    }
                }
                for (si, (bst, tst)) in bb.storeys.iter().zip(tb.storeys.iter()).enumerate() {
                    if bst.permanent_gk_n.to_bits() != tst.permanent_gk_n.to_bits() {
                        mutations.push(En1998Mutation::ChangeStoreyPermanentGkN(change_storey_permanent_gk_n::ChangeStoreyPermanentGkN {
                            building_index: bi,
                            storey_index: si,
                            new_permanent_gk_n: tst.permanent_gk_n,
                        }));
                    }
                    if bst.stiffness_x.to_bits() != tst.stiffness_x.to_bits() {
                        mutations.push(En1998Mutation::ChangeStoreyStiffnessX(change_storey_stiffness_x::ChangeStoreyStiffnessX {
                            building_index: bi,
                            storey_index: si,
                            new_stiffness_x: tst.stiffness_x,
                        }));
                    }
                    if bst.drift_x_m.to_bits() != tst.drift_x_m.to_bits() {
                        mutations.push(En1998Mutation::ChangeStoreyDriftXM(change_storey_drift_xm::ChangeStoreyDriftXM {
                            building_index: bi,
                            storey_index: si,
                            new_drift_x_m: tst.drift_x_m,
                        }));
                    }
                }
                for (mi, (bm, tm)) in bb.members.iter().zip(tb.members.iter()).enumerate() {
                    if bm.detailing_compatible_with_q != tm.detailing_compatible_with_q {
                        mutations.push(En1998Mutation::ChangeMemberDetailing(change_member_detailing::ChangeMemberDetailing {
                            building_index: bi,
                            member_index: mi,
                            new_detailing_compatible_with_q: tm.detailing_compatible_with_q,
                        }));
                    }
                }
            }
        }
        if base.bridges != target.bridges {
            if base.bridges.len() == target.bridges.len() && base.bridges.iter().zip(&target.bridges).all(|(b, t)| crate::En1998Bridge { v_rd_n: t.v_rd_n, ..b.clone() } == *t) {
                for (index, (b, t)) in base.bridges.iter().zip(&target.bridges).enumerate() {
                    if b.v_rd_n.to_bits() != t.v_rd_n.to_bits() {
                        mutations.push(change_bridge_v_rd_n::ChangeBridgeVRdN { index, new_v_rd_n: t.v_rd_n }.into());
                    }
                }
            } else {
                mutations.extend(replace_all(&base.bridges, &target.bridges, |index| remove_bridge::RemoveBridge { index }.into(), |index, bridge| insert_bridge::InsertBridge { index, bridge }.into()));
            }
        }
        if base.assessments != target.assessments {
            if base.assessments.len() == target.assessments.len() && base.assessments.iter().zip(&target.assessments).all(|(b, t)| crate::En1998Assessment { r_k_n: t.r_k_n, ..b.clone() } == *t) {
                for (index, (b, t)) in base.assessments.iter().zip(&target.assessments).enumerate() {
                    if b.r_k_n.to_bits() != t.r_k_n.to_bits() {
                        mutations.push(change_assessment_rkn::ChangeAssessmentRKN { index, new_r_k_n: t.r_k_n }.into());
                    }
                }
            } else {
                mutations.extend(replace_all(&base.assessments, &target.assessments, |index| remove_assessment::RemoveAssessment { index }.into(), |index, assessment| insert_assessment::InsertAssessment { index, assessment }.into()));
            }
        }
        if base.silos != target.silos {
            mutations.extend(replace_all(&base.silos, &target.silos, |index| remove_silo::RemoveSilo { index }.into(), |index, silo| insert_silo::InsertSilo { index, silo }.into()));
        }
        if base.tanks != target.tanks {
            mutations.extend(replace_all(&base.tanks, &target.tanks, |index| remove_tank::RemoveTank { index }.into(), |index, tank| insert_tank::InsertTank { index, tank }.into()));
        }
        if base.foundations != target.foundations {
            mutations.extend(replace_all(&base.foundations, &target.foundations, |index| remove_foundation::RemoveFoundation { index }.into(), |index, foundation| insert_foundation::InsertFoundation { index, foundation }.into()));
        }
        if base.retaining_walls != target.retaining_walls {
            mutations.extend(replace_all(&base.retaining_walls, &target.retaining_walls, |index| remove_retaining_wall::RemoveRetainingWall { index }.into(), |index, wall| insert_retaining_wall::InsertRetainingWall { index, wall }.into()));
        }
        if base.towers != target.towers {
            if base.towers.len() == target.towers.len() && base.towers.iter().zip(&target.towers).all(|(b, t)| crate::En1998Tower { m_rd_nm: t.m_rd_nm, ..b.clone() } == *t) {
                for (index, (b, t)) in base.towers.iter().zip(&target.towers).enumerate() {
                    if b.m_rd_nm.to_bits() != t.m_rd_nm.to_bits() {
                        mutations.push(change_tower_m_rd_nm::ChangeTowerMRdNm { index, new_m_rd_nm: t.m_rd_nm }.into());
                    }
                }
            } else {
                mutations.extend(replace_all(&base.towers, &target.towers, |index| remove_tower::RemoveTower { index }.into(), |index, tower| insert_tower::InsertTower { index, tower }.into()));
            }
        }
        mutations
    }
}

/// 🔁 Replaces a whole collection through the closed vocabulary: every base record removed back to front, then every target record inserted in order.
fn replace_all<T: Clone>(base: &[T], target: &[T], remove: impl Fn(usize) -> En1998Mutation, insert: impl Fn(usize, T) -> En1998Mutation) -> Vec<En1998Mutation> {
    (0..base.len()).rev().map(&remove).chain(target.iter().cloned().enumerate().map(|(index, item)| insert(index, item))).collect()
}

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes one committed mutation JSON document into [`En1998Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1998_mutation_json(text: &str) -> Result<En1998Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_en1998_mutation(base: &En1998Snapshot, mutation: &En1998Mutation) -> Result<(En1998Snapshot, Vec<String>), String> {
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_en1998_mutation(mutation: &En1998Mutation, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests
