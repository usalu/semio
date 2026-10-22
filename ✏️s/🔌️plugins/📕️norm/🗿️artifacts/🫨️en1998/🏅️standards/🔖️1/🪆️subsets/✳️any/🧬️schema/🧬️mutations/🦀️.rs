//! 🧬️ En1998 artifact — closed semantic mutation dispatch enum.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1998_diff::En1998Diff;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::EDIT_RULES;

use semio_s_artifact_norm_en1998_mut_buildings::change_annex;
use semio_s_artifact_norm_en1998_mut_buildings::update_site;
use semio_s_artifact_norm_en1998_mut_buildings::insert_building;
use semio_s_artifact_norm_en1998_mut_buildings::remove_building;
use semio_s_artifact_norm_en1998_mut_buildings::change_system_v_rd_n;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_permanent_gk_n;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_stiffness_x;
use semio_s_artifact_norm_en1998_mut_buildings::change_storey_drift_xm;
use semio_s_artifact_norm_en1998_mut_buildings::change_building_plan_regular;
use semio_s_artifact_norm_en1998_mut_buildings::change_elevation_regular;
use semio_s_artifact_norm_en1998_mut_members::change_member_detailing;
use semio_s_artifact_norm_en1998_mut_members::change_masonry_wall_ratio;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_bridge;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_bridge_v_rd_n;
use semio_s_artifact_norm_en1998_mut_members::insert_assessment;
use semio_s_artifact_norm_en1998_mut_members::change_assessment_rkn;
use semio_s_artifact_norm_en1998_mut_silos_tanks::insert_silo;
use semio_s_artifact_norm_en1998_mut_silos_tanks::insert_tank;
use semio_s_artifact_norm_en1998_mut_foundations::insert_foundation;
use semio_s_artifact_norm_en1998_mut_foundations::insert_retaining_wall;
use semio_s_artifact_norm_en1998_mut_bridges_towers::insert_tower;
use semio_s_artifact_norm_en1998_mut_bridges_towers::change_tower_m_rd_nm;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_bridge;
use semio_s_artifact_norm_en1998_mut_members::remove_assessment;
use semio_s_artifact_norm_en1998_mut_silos_tanks::remove_silo;
use semio_s_artifact_norm_en1998_mut_silos_tanks::remove_tank;
use semio_s_artifact_norm_en1998_mut_foundations::remove_foundation;
use semio_s_artifact_norm_en1998_mut_foundations::remove_retaining_wall;
use semio_s_artifact_norm_en1998_mut_bridges_towers::remove_tower;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1998Snapshot, diff = En1998Diff, schema = "s.norm.en1998", reference = En1998Snapshot, reference_diff = En1998Diff)]
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
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_en1998_mutation(base: &semio_s_artifact_norm_en1998_model::En1998Snapshot, mutation: &crate::En1998Mutation) -> Result<(semio_s_artifact_norm_en1998_model::En1998Snapshot, Vec<String>), String> {
    let raised = <crate::En1998Mutation as protocol::ColdMutation<semio_s_artifact_norm_en1998_model::En1998Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_en1998_mutation(mutation: &crate::En1998Mutation, base: &semio_s_artifact_norm_en1998_model::En1998Snapshot) -> Result<Vec<crate::En1998Mutation>, semio_framework_value::ValueError> {
    <crate::En1998Mutation as protocol::ColdMutation<semio_s_artifact_norm_en1998_model::En1998Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;
