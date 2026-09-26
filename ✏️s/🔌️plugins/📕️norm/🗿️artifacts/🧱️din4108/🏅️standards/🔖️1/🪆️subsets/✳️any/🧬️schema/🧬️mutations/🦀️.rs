//! 🧬️ Din4108 mutations — envelope subject vocabulary.

use crate::{Din4108Diff, Din4108Snapshot};

use super::change_climate_zone;
use super::change_usage;
use super::change_t_int_c;
use super::change_rh_int;
use super::change_airtightness_n50;
use super::change_has_mechanical_ventilation;
use super::change_bb2_details_conform;
use super::insert_zone;
use super::remove_zone;
use super::change_zone_floor_area;
use super::change_zone_heaviness;
use super::change_zone_night_ventilation;
use super::insert_zone_window;
use super::remove_zone_window;
use super::change_zone_window_area;
use super::change_zone_window_g_value;
use super::change_zone_window_shading_fc;
use super::insert_element;
use super::remove_element;
use super::change_element_area;
use super::change_element_adjacent;
use super::change_element_kind;
use super::insert_layer;
use super::remove_layer;
use super::reorder_layers;
use super::change_layer_thickness;
use super::change_layer_lambda;
use super::change_layer_mu;
use super::change_layer_material_id;
use super::insert_thermal_bridge;
use super::remove_thermal_bridge;
use super::change_thermal_bridge_psi;
use super::change_thermal_bridge_length;
use super::change_element_orientation_deg;
use super::change_element_inclination_deg;
use super::change_element_delta_u_g;
use super::change_element_delta_u_f;
use super::change_element_delta_u_r;
use super::change_thermal_bridge_bb2_type;
use super::change_zone_window_orientation;
use super::change_zone_window_inclination_deg;
use super::change_layer_application_type;
use super::change_layer_compressive_class;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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

impl Din4108Mutation {
    pub fn from_snapshot(base: &Din4108Snapshot, target: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        let mut mutations = Vec::new();
        if base.climate_zone != target.climate_zone {
            mutations.push(Din4108Mutation::ChangeClimateZone(change_climate_zone::ChangeClimateZone { new_climate_zone: target.climate_zone }));
        }
        if base.usage != target.usage {
            mutations.push(Din4108Mutation::ChangeUsage(change_usage::ChangeUsage { new_usage: target.usage.clone() }));
        }
        if base.t_int_c.to_bits() != target.t_int_c.to_bits() {
            mutations.push(Din4108Mutation::ChangeTIntC(change_t_int_c::ChangeTIntC { new_t_int_c: target.t_int_c }));
        }
        if base.rh_int.to_bits() != target.rh_int.to_bits() {
            mutations.push(Din4108Mutation::ChangeRhInt(change_rh_int::ChangeRhInt { new_rh_int: target.rh_int }));
        }
        if base.airtightness_n50.to_bits() != target.airtightness_n50.to_bits() {
            mutations.push(Din4108Mutation::ChangeAirtightnessN50(change_airtightness_n50::ChangeAirtightnessN50 { new_airtightness_n50: target.airtightness_n50 }));
        }
        if base.has_mechanical_ventilation != target.has_mechanical_ventilation {
            mutations.push(Din4108Mutation::ChangeHasMechanicalVentilation(change_has_mechanical_ventilation::ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: target.has_mechanical_ventilation }));
        }
        if base.bb2_details_conform != target.bb2_details_conform {
            mutations.push(Din4108Mutation::ChangeBb2DetailsConform(change_bb2_details_conform::ChangeBb2DetailsConform { new_bb2_details_conform: target.bb2_details_conform }));
        }
        if base.zones != target.zones {
            for index in (0..base.zones.len()).rev() {
                mutations.push(Din4108Mutation::RemoveZone(remove_zone::RemoveZone { index }));
            }
            for (index, zone) in target.zones.iter().enumerate() {
                mutations.push(Din4108Mutation::InsertZone(insert_zone::InsertZone { index, zone: zone.clone() }));
            }
        }
        if base.elements != target.elements {
            for index in (0..base.elements.len()).rev() {
                mutations.push(Din4108Mutation::RemoveElement(remove_element::RemoveElement { index }));
            }
            for (index, element) in target.elements.iter().enumerate() {
                mutations.push(Din4108Mutation::InsertElement(insert_element::InsertElement { index, element: element.clone() }));
            }
        }
        if base.thermal_bridges != target.thermal_bridges {
            for index in (0..base.thermal_bridges.len()).rev() {
                mutations.push(Din4108Mutation::RemoveThermalBridge(remove_thermal_bridge::RemoveThermalBridge { index }));
            }
            for (index, bridge) in target.thermal_bridges.iter().enumerate() {
                mutations.push(Din4108Mutation::InsertThermalBridge(insert_thermal_bridge::InsertThermalBridge { index, bridge: bridge.clone() }));
            }
        }
        mutations
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

pub fn decode_din4108_mutation_json(text: &str) -> Result<Din4108Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn apply_din4108_mutation(base: &Din4108Snapshot, mutation: &Din4108Mutation) -> Result<(Din4108Snapshot, Vec<String>), String> {
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

pub fn inverse_din4108_mutation(mutation: &Din4108Mutation, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(mutation, base)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
