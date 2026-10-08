//! 🧬️ En1998 artifact — closed semantic mutation dispatch enum.

use crate::diff::En1998Diff;
use crate::En1998Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::EDIT_RULES;
pub use crate::standards::v1::subsets::any::io::mutation_bridge::{apply_en1998_mutation, inverse_en1998_mutation};

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

/// 🔁 Replaces a whole collection through the closed vocabulary: every base record removed back to front, then every target record inserted in order.
fn replace_all<T: Clone>(base: &[T], target: &[T], remove: impl Fn(usize) -> En1998Mutation, insert: impl Fn(usize, T) -> En1998Mutation) -> Vec<En1998Mutation> {
    (0..base.len()).rev().map(&remove).chain(target.iter().cloned().enumerate().map(|(index, item)| insert(index, item))).collect()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;


#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
