//! 🧬️ Din18599 artifact — closed semantic mutation dispatch enum.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES, ID_REMOVALS};

use semio_s_artifact_norm_din18599_mut_building::change_building_category;
use semio_s_artifact_norm_din18599_mut_building::change_attachment;
use semio_s_artifact_norm_din18599_mut_building::change_use_class;
use semio_s_artifact_norm_din18599_mut_building::change_method;
use semio_s_artifact_norm_din18599_mut_building::change_net_floor_area_m2;
use semio_s_artifact_norm_din18599_mut_building::change_heated_volume_m3;
use semio_s_artifact_norm_din18599_mut_building::change_geg_qp_factor;
use semio_s_artifact_norm_din18599_mut_building::change_delta_u_wb;
use semio_s_artifact_norm_din18599_mut_building::change_automation_class;
use semio_s_artifact_norm_din18599_mut_systems::specify_heating_system;
use semio_s_artifact_norm_din18599_mut_systems::specify_dhw_system;
use semio_s_artifact_norm_din18599_mut_systems::update_ventilation;
use semio_s_artifact_norm_din18599_mut_systems::update_cooling;
use semio_s_artifact_norm_din18599_mut_systems::update_lighting;
use semio_s_artifact_norm_din18599_mut_systems::update_renewables;
use semio_s_artifact_norm_din18599_mut_zoning::change_element_u;
use semio_s_artifact_norm_din18599_mut_zone_rows::insert_zone;
use semio_s_artifact_norm_din18599_mut_zone_rows::remove_zone;
use semio_s_artifact_norm_din18599_mut_zone_rows::change_zone_label_en;
use semio_s_artifact_norm_din18599_mut_zone_rows::change_zone_label_de;
use semio_s_artifact_norm_din18599_mut_zone_rows::change_zone_usage_profile;
use semio_s_artifact_norm_din18599_mut_zone_rows::change_zone_occupants;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_area_m2;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_volume_m3;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_theta_i_heat_c;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_theta_i_cool_c;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_internal_gains_w_m2;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_lighting_power_w_m2;
use semio_s_artifact_norm_din18599_mut_zone_fields::change_zone_thermal_mass;
use semio_s_artifact_norm_din18599_mut_element_rows::insert_element;
use semio_s_artifact_norm_din18599_mut_element_rows::remove_element;
use semio_s_artifact_norm_din18599_mut_element_rows::change_element_label_en;
use semio_s_artifact_norm_din18599_mut_element_rows::change_element_label_de;
use semio_s_artifact_norm_din18599_mut_element_rows::change_element_kind;
use semio_s_artifact_norm_din18599_mut_element_rows::change_element_zone_id;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_area_m2;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_orientation_deg;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_tilt_deg;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_g_value;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_fc;
use semio_s_artifact_norm_din18599_mut_element_fields::change_element_adjacency;
use semio_s_artifact_norm_din18599_mut_zoning::update_climate;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din18599 building subject.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din18599Snapshot, diff = Din18599Diff, schema = "norm.din18599", reference = Din18599Snapshot, reference_diff = Din18599Diff)]
pub enum Din18599Mutation {
    ChangeBuildingCategory(change_building_category::ChangeBuildingCategory),
    ChangeAttachment(change_attachment::ChangeAttachment),
    ChangeUseClass(change_use_class::ChangeUseClass),
    ChangeMethod(change_method::ChangeMethod),
    ChangeNetFloorAreaM2(change_net_floor_area_m2::ChangeNetFloorAreaM2),
    ChangeHeatedVolumeM3(change_heated_volume_m3::ChangeHeatedVolumeM3),
    ChangeGegQpFactor(change_geg_qp_factor::ChangeGegQpFactor),
    ChangeDeltaUWb(change_delta_u_wb::ChangeDeltaUWb),
    ChangeAutomationClass(change_automation_class::ChangeAutomationClass),
    SpecifyHeatingSystem(specify_heating_system::SpecifyHeatingSystem),
    SpecifyDhwSystem(specify_dhw_system::SpecifyDhwSystem),
    UpdateVentilation(update_ventilation::UpdateVentilation),
    UpdateCooling(update_cooling::UpdateCooling),
    UpdateLighting(update_lighting::UpdateLighting),
    UpdateRenewables(update_renewables::UpdateRenewables),
    InsertZone(insert_zone::InsertZone),
    RemoveZone(remove_zone::RemoveZone),
    ChangeZoneLabelEn(change_zone_label_en::ChangeZoneLabelEn),
    ChangeZoneLabelDe(change_zone_label_de::ChangeZoneLabelDe),
    ChangeZoneUsageProfile(change_zone_usage_profile::ChangeZoneUsageProfile),
    ChangeZoneOccupants(change_zone_occupants::ChangeZoneOccupants),
    ChangeZoneAreaM2(change_zone_area_m2::ChangeZoneAreaM2),
    ChangeZoneVolumeM3(change_zone_volume_m3::ChangeZoneVolumeM3),
    ChangeZoneThetaIHeatC(change_zone_theta_i_heat_c::ChangeZoneThetaIHeatC),
    ChangeZoneThetaICoolC(change_zone_theta_i_cool_c::ChangeZoneThetaICoolC),
    ChangeZoneInternalGainsWM2(change_zone_internal_gains_w_m2::ChangeZoneInternalGainsWM2),
    ChangeZoneLightingPowerWM2(change_zone_lighting_power_w_m2::ChangeZoneLightingPowerWM2),
    ChangeZoneThermalMass(change_zone_thermal_mass::ChangeZoneThermalMass),
    InsertElement(insert_element::InsertElement),
    RemoveElement(remove_element::RemoveElement),
    ChangeElementLabelEn(change_element_label_en::ChangeElementLabelEn),
    ChangeElementLabelDe(change_element_label_de::ChangeElementLabelDe),
    ChangeElementKind(change_element_kind::ChangeElementKind),
    ChangeElementZoneId(change_element_zone_id::ChangeElementZoneId),
    ChangeElementAreaM2(change_element_area_m2::ChangeElementAreaM2),
    ChangeElementOrientationDeg(change_element_orientation_deg::ChangeElementOrientationDeg),
    ChangeElementTiltDeg(change_element_tilt_deg::ChangeElementTiltDeg),
    ChangeElementGValue(change_element_g_value::ChangeElementGValue),
    ChangeElementFc(change_element_fc::ChangeElementFc),
    ChangeElementAdjacency(change_element_adjacency::ChangeElementAdjacency),
    ChangeElementU(change_element_u::ChangeElementU),
    UpdateClimate(update_climate::UpdateClimate),
}

pub const KINDS: &[&str] = &[
    "change-building-category",
    "change-attachment",
    "change-use-class",
    "change-method",
    "change-net-floor-area-m2",
    "change-heated-volume-m3",
    "change-geg-qp-factor",
    "change-delta-u-wb",
    "change-automation-class",
    "specify-heating-system",
    "specify-dhw-system",
    "update-ventilation",
    "update-cooling",
    "update-lighting",
    "update-renewables",
    "insert-zone",
    "remove-zone",
    "change-zone-label-en",
    "change-zone-label-de",
    "change-zone-usage-profile",
    "change-zone-occupants",
    "change-zone-area-m2",
    "change-zone-volume-m3",
    "change-zone-theta-i-heat-c",
    "change-zone-theta-i-cool-c",
    "change-zone-internal-gains-w-m2",
    "change-zone-lighting-power-w-m2",
    "change-zone-thermal-mass",
    "insert-element",
    "remove-element",
    "change-element-label-en",
    "change-element-label-de",
    "change-element-kind",
    "change-element-zone-id",
    "change-element-area-m2",
    "change-element-orientation-deg",
    "change-element-tilt-deg",
    "change-element-g-value",
    "change-element-fc",
    "change-element-adjacency",
    "change-element-u",
    "update-climate",
];
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::Din18599Mutation;
    use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_din18599_mutation(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Result<(Din18599Snapshot, Vec<String>), String> {
        let raised = <Din18599Mutation as protocol::ColdMutation<Din18599Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_din18599_mutation(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
        <Din18599Mutation as protocol::ColdMutation<Din18599Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;
