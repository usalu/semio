//! 🧬️ Din4108 mutations — envelope subject vocabulary.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din4108_diff::Din4108Diff;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::EDIT_RULES;

use semio_s_artifact_norm_din4108_mut_building::change_climate_zone;
use semio_s_artifact_norm_din4108_mut_building::change_usage;
use semio_s_artifact_norm_din4108_mut_building::change_t_int_c;
use semio_s_artifact_norm_din4108_mut_building::change_rh_int;
use semio_s_artifact_norm_din4108_mut_building::change_airtightness_n50;
use semio_s_artifact_norm_din4108_mut_building::change_has_mechanical_ventilation;
use semio_s_artifact_norm_din4108_mut_building::change_bb2_details_conform;
use semio_s_artifact_norm_din4108_mut_zones::insert_zone;
use semio_s_artifact_norm_din4108_mut_zones::remove_zone;
use semio_s_artifact_norm_din4108_mut_zones::change_zone_floor_area;
use semio_s_artifact_norm_din4108_mut_zones::change_zone_heaviness;
use semio_s_artifact_norm_din4108_mut_zones::change_zone_night_ventilation;
use semio_s_artifact_norm_din4108_mut_windows::insert_zone_window;
use semio_s_artifact_norm_din4108_mut_windows::remove_zone_window;
use semio_s_artifact_norm_din4108_mut_windows::change_zone_window_area;
use semio_s_artifact_norm_din4108_mut_windows::change_zone_window_g_value;
use semio_s_artifact_norm_din4108_mut_windows::change_zone_window_shading_fc;
use semio_s_artifact_norm_din4108_mut_elements::insert_element;
use semio_s_artifact_norm_din4108_mut_elements::remove_element;
use semio_s_artifact_norm_din4108_mut_elements::change_element_area;
use semio_s_artifact_norm_din4108_mut_elements::change_element_adjacent;
use semio_s_artifact_norm_din4108_mut_elements::change_element_kind;
use semio_s_artifact_norm_din4108_mut_layers::insert_layer;
use semio_s_artifact_norm_din4108_mut_layers::remove_layer;
use semio_s_artifact_norm_din4108_mut_layers::reorder_layers;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_thickness;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_lambda;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_mu;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_material_id;
use semio_s_artifact_norm_din4108_mut_thermal_bridge::insert_thermal_bridge;
use semio_s_artifact_norm_din4108_mut_thermal_bridge::remove_thermal_bridge;
use semio_s_artifact_norm_din4108_mut_thermal_bridge::change_thermal_bridge_psi;
use semio_s_artifact_norm_din4108_mut_thermal_bridge::change_thermal_bridge_length;
use semio_s_artifact_norm_din4108_mut_elements::change_element_orientation_deg;
use semio_s_artifact_norm_din4108_mut_elements::change_element_inclination_deg;
use semio_s_artifact_norm_din4108_mut_elements::change_element_delta_u_g;
use semio_s_artifact_norm_din4108_mut_elements::change_element_delta_u_f;
use semio_s_artifact_norm_din4108_mut_elements::change_element_delta_u_r;
use semio_s_artifact_norm_din4108_mut_thermal_bridge::change_thermal_bridge_bb2_type;
use semio_s_artifact_norm_din4108_mut_windows::change_zone_window_orientation;
use semio_s_artifact_norm_din4108_mut_windows::change_zone_window_inclination_deg;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_application_type;
use semio_s_artifact_norm_din4108_mut_layers::change_layer_compressive_class;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = Din4108Snapshot, diff = Din4108Diff, schema = "s.norm.din4108")]
pub enum Din4108Mutation {
    ChangeClimateZone(change_climate_zone::ChangeClimateZone),
    ChangeUsage(change_usage::ChangeUsage),
    ChangeTIntC(change_t_int_c::ChangeTIntC),
    ChangeRhInt(change_rh_int::ChangeRhInt),
    ChangeAirtightnessN50(change_airtightness_n50::ChangeAirtightnessN50),
    ChangeHasMechanicalVentilation(change_has_mechanical_ventilation::ChangeHasMechanicalVentilation),
    ChangeBb2DetailsConform(change_bb2_details_conform::ChangeBb2DetailsConform),
    InsertZone(insert_zone::InsertZone),
    RemoveZone(remove_zone::RemoveZone),
    ChangeZoneFloorArea(change_zone_floor_area::ChangeZoneFloorArea),
    ChangeZoneHeaviness(change_zone_heaviness::ChangeZoneHeaviness),
    ChangeZoneNightVentilation(change_zone_night_ventilation::ChangeZoneNightVentilation),
    InsertZoneWindow(insert_zone_window::InsertZoneWindow),
    RemoveZoneWindow(remove_zone_window::RemoveZoneWindow),
    ChangeZoneWindowArea(change_zone_window_area::ChangeZoneWindowArea),
    ChangeZoneWindowGValue(change_zone_window_g_value::ChangeZoneWindowGValue),
    ChangeZoneWindowShadingFc(change_zone_window_shading_fc::ChangeZoneWindowShadingFc),
    InsertElement(insert_element::InsertElement),
    RemoveElement(remove_element::RemoveElement),
    ChangeElementArea(change_element_area::ChangeElementArea),
    ChangeElementAdjacent(change_element_adjacent::ChangeElementAdjacent),
    ChangeElementKind(change_element_kind::ChangeElementKind),
    InsertLayer(insert_layer::InsertLayer),
    RemoveLayer(remove_layer::RemoveLayer),
    ReorderLayers(reorder_layers::ReorderLayers),
    ChangeLayerThickness(change_layer_thickness::ChangeLayerThickness),
    ChangeLayerLambda(change_layer_lambda::ChangeLayerLambda),
    ChangeLayerMu(change_layer_mu::ChangeLayerMu),
    ChangeLayerMaterialId(change_layer_material_id::ChangeLayerMaterialId),
    InsertThermalBridge(insert_thermal_bridge::InsertThermalBridge),
    RemoveThermalBridge(remove_thermal_bridge::RemoveThermalBridge),
    ChangeThermalBridgePsi(change_thermal_bridge_psi::ChangeThermalBridgePsi),
    ChangeThermalBridgeLength(change_thermal_bridge_length::ChangeThermalBridgeLength),
    ChangeElementOrientationDeg(change_element_orientation_deg::ChangeElementOrientationDeg),
    ChangeElementInclinationDeg(change_element_inclination_deg::ChangeElementInclinationDeg),
    ChangeElementDeltaUg(change_element_delta_u_g::ChangeElementDeltaUg),
    ChangeElementDeltaUf(change_element_delta_u_f::ChangeElementDeltaUf),
    ChangeElementDeltaUr(change_element_delta_u_r::ChangeElementDeltaUr),
    ChangeThermalBridgeBb2Type(change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type),
    ChangeZoneWindowOrientation(change_zone_window_orientation::ChangeZoneWindowOrientation),
    ChangeZoneWindowInclinationDeg(change_zone_window_inclination_deg::ChangeZoneWindowInclinationDeg),
    ChangeLayerApplicationType(change_layer_application_type::ChangeLayerApplicationType),
    ChangeLayerCompressiveClass(change_layer_compressive_class::ChangeLayerCompressiveClass),
}

pub const KINDS: &[&str] = &[
    "change-climate-zone",
    "change-usage",
    "change-t-int-c",
    "change-rh-int",
    "change-airtightness-n50",
    "change-has-mechanical-ventilation",
    "change-bb2-details-conform",
    "insert-zone",
    "remove-zone",
    "change-zone-floor-area",
    "change-zone-heaviness",
    "change-zone-night-ventilation",
    "insert-zone-window",
    "remove-zone-window",
    "change-zone-window-area",
    "change-zone-window-g-value",
    "change-zone-window-shading-fc",
    "insert-element",
    "remove-element",
    "change-element-area",
    "change-element-adjacent",
    "change-element-kind",
    "insert-layer",
    "remove-layer",
    "reorder-layers",
    "change-layer-thickness",
    "change-layer-lambda",
    "change-layer-mu",
    "change-layer-material-id",
    "insert-thermal-bridge",
    "remove-thermal-bridge",
    "change-thermal-bridge-psi",
    "change-thermal-bridge-length",
    "change-element-orientation-deg",
    "change-element-inclination-deg",
    "change-element-delta-ug",
    "change-element-delta-uf",
    "change-element-delta-ur",
    "change-thermal-bridge-bb2-type",
    "change-zone-window-orientation",
    "change-zone-window-inclination-deg",
    "change-layer-application-type",
    "change-layer-compressive-class",
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

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_din4108_mutation(base: &semio_s_artifact_norm_din4108_model::Din4108Snapshot, mutation: &crate::Din4108Mutation) -> Result<(semio_s_artifact_norm_din4108_model::Din4108Snapshot, Vec<String>), String> {
    let raised = <crate::Din4108Mutation as protocol::Mutation<semio_s_artifact_norm_din4108_model::Din4108Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_din4108_mutation(mutation: &crate::Din4108Mutation, base: &semio_s_artifact_norm_din4108_model::Din4108Snapshot) -> Result<Vec<crate::Din4108Mutation>, semio_framework_value::ValueError> {
    <crate::Din4108Mutation as protocol::Mutation<semio_s_artifact_norm_din4108_model::Din4108Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;
