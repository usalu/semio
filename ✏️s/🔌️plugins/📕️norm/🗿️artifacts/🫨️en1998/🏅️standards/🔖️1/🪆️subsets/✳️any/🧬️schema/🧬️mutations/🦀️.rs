//! 🧬️ En1998 artifact — closed semantic mutation dispatch enum.

use crate::diff::En1998Diff;
use crate::En1998Snapshot;

use super::change_annex;
use super::update_site;
use super::insert_building;
use super::remove_building;
use super::change_system_base_shear_resistance_n;
use super::change_storey_permanent_gk_n;
use super::change_storey_stiffness_x;
use super::change_storey_drift_xm;
use super::change_building_plan_regular;
use super::change_building_elevation_regular;
use super::change_member_detailing_compatible;
use super::change_building_masonry_wall_area_ratio;
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
    ChangeSystemBaseShearResistanceN(change_system_base_shear_resistance_n::ChangeSystemBaseShearResistanceN),
    ChangeStoreyPermanentGkN(change_storey_permanent_gk_n::ChangeStoreyPermanentGkN),
    ChangeStoreyStiffnessX(change_storey_stiffness_x::ChangeStoreyStiffnessX),
    ChangeStoreyDriftXM(change_storey_drift_xm::ChangeStoreyDriftXM),
    ChangeBuildingPlanRegular(change_building_plan_regular::ChangeBuildingPlanRegular),
    ChangeBuildingElevationRegular(change_building_elevation_regular::ChangeBuildingElevationRegular),
    ChangeMemberDetailingCompatible(change_member_detailing_compatible::ChangeMemberDetailingCompatible),
    ChangeBuildingMasonryWallAreaRatio(change_building_masonry_wall_area_ratio::ChangeBuildingMasonryWallAreaRatio),
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
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "update-site",
    "insert-building",
    "remove-building",
    "change-system-base-shear-resistance-n",
    "change-storey-permanent-gk-n",
    "change-storey-stiffness-x",
    "change-storey-drift-xm",
    "change-building-plan-regular",
    "change-building-elevation-regular",
    "change-member-detailing-compatible",
    "change-building-masonry-wall-area-ratio",
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
                    mutations.push(En1998Mutation::ChangeBuildingElevationRegular(change_building_elevation_regular::ChangeBuildingElevationRegular {
                        building_index: bi,
                        new_elevation_regular: tb.elevation_regular,
                    }));
                }
                if bb.masonry_wall_area_ratio.to_bits() != tb.masonry_wall_area_ratio.to_bits() {
                    mutations.push(En1998Mutation::ChangeBuildingMasonryWallAreaRatio(change_building_masonry_wall_area_ratio::ChangeBuildingMasonryWallAreaRatio {
                        building_index: bi,
                        new_masonry_wall_area_ratio: tb.masonry_wall_area_ratio,
                    }));
                }
                for (si, (bs, ts)) in bb.systems.iter().zip(tb.systems.iter()).enumerate() {
                    if bs.base_shear_resistance_n.to_bits() != ts.base_shear_resistance_n.to_bits() {
                        mutations.push(En1998Mutation::ChangeSystemBaseShearResistanceN(change_system_base_shear_resistance_n::ChangeSystemBaseShearResistanceN {
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
                        mutations.push(En1998Mutation::ChangeMemberDetailingCompatible(change_member_detailing_compatible::ChangeMemberDetailingCompatible {
                            building_index: bi,
                            member_index: mi,
                            new_detailing_compatible_with_q: tm.detailing_compatible_with_q,
                        }));
                    }
                }
            }
        }
        if base.bridges != target.bridges {
            if base.bridges.len() == target.bridges.len() {
                for (i, (b, tbridge)) in base.bridges.iter().zip(target.bridges.iter()).enumerate() {
                    if b.v_rd_n.to_bits() != tbridge.v_rd_n.to_bits() {
                        mutations.push(En1998Mutation::ChangeBridgeVRdN(change_bridge_v_rd_n::ChangeBridgeVRdN { index: i, new_v_rd_n: tbridge.v_rd_n }));
                    }
                }
            } else {
                for (index, bridge) in target.bridges.iter().enumerate() {
                    mutations.push(En1998Mutation::InsertBridge(insert_bridge::InsertBridge { index, bridge: bridge.clone() }));
                }
            }
        }
        if base.assessments != target.assessments {
            if base.assessments.len() == target.assessments.len() {
                for (i, (b, a)) in base.assessments.iter().zip(target.assessments.iter()).enumerate() {
                    if b.r_k_n.to_bits() != a.r_k_n.to_bits() {
                        mutations.push(En1998Mutation::ChangeAssessmentRKN(change_assessment_rkn::ChangeAssessmentRKN { index: i, new_r_k_n: a.r_k_n }));
                    }
                }
            } else {
                for (index, assessment) in target.assessments.iter().enumerate() {
                    mutations.push(En1998Mutation::InsertAssessment(insert_assessment::InsertAssessment { index, assessment: assessment.clone() }));
                }
            }
        }
        if base.silos != target.silos {
            for (index, silo) in target.silos.iter().enumerate() {
                if base.silos.get(index) != Some(silo) {
                    mutations.push(En1998Mutation::InsertSilo(insert_silo::InsertSilo { index, silo: silo.clone() }));
                }
            }
        }
        if base.tanks != target.tanks {
            for (index, tank) in target.tanks.iter().enumerate() {
                if base.tanks.get(index) != Some(tank) {
                    mutations.push(En1998Mutation::InsertTank(insert_tank::InsertTank { index, tank: tank.clone() }));
                }
            }
        }
        if base.foundations != target.foundations {
            for (index, foundation) in target.foundations.iter().enumerate() {
                if base.foundations.get(index) != Some(foundation) {
                    mutations.push(En1998Mutation::InsertFoundation(insert_foundation::InsertFoundation { index, foundation: foundation.clone() }));
                }
            }
        }
        if base.retaining_walls != target.retaining_walls {
            for (index, wall) in target.retaining_walls.iter().enumerate() {
                if base.retaining_walls.get(index) != Some(wall) {
                    mutations.push(En1998Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index, wall: wall.clone() }));
                }
            }
        }
        if base.towers != target.towers {
            if base.towers.len() == target.towers.len() {
                for (i, (b, tw)) in base.towers.iter().zip(target.towers.iter()).enumerate() {
                    if b.m_rd_nm.to_bits() != tw.m_rd_nm.to_bits() {
                        mutations.push(En1998Mutation::ChangeTowerMRdNm(change_tower_m_rd_nm::ChangeTowerMRdNm { index: i, new_m_rd_nm: tw.m_rd_nm }));
                    }
                }
            } else {
                for (index, tower) in target.towers.iter().enumerate() {
                    mutations.push(En1998Mutation::InsertTower(insert_tower::InsertTower { index, tower: tower.clone() }));
                }
            }
        }
        mutations
    }
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
