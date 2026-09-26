//! Fixture corpus — regenerate + assert every KINDS entry has a triad.

use crate::standards::v1::subsets::any::schema::mutations::{
    apply_din4108_mutation, Din4108Mutation, KINDS,
};
use crate::standards::v1::subsets::any::schema::mutations::change_airtightness_n50::ChangeAirtightnessN50;
use crate::standards::v1::subsets::any::schema::mutations::change_bb2_details_conform::ChangeBb2DetailsConform;
use crate::standards::v1::subsets::any::schema::mutations::change_climate_zone::ChangeClimateZone;
use crate::standards::v1::subsets::any::schema::mutations::change_element_adjacent::ChangeElementAdjacent;
use crate::standards::v1::subsets::any::schema::mutations::change_element_area::ChangeElementArea;
use crate::standards::v1::subsets::any::schema::mutations::change_element_delta_u_f::ChangeElementDeltaUf;
use crate::standards::v1::subsets::any::schema::mutations::change_element_delta_u_g::ChangeElementDeltaUg;
use crate::standards::v1::subsets::any::schema::mutations::change_element_delta_u_r::ChangeElementDeltaUr;
use crate::standards::v1::subsets::any::schema::mutations::change_element_inclination_deg::ChangeElementInclinationDeg;
use crate::standards::v1::subsets::any::schema::mutations::change_element_kind::ChangeElementKind;
use crate::standards::v1::subsets::any::schema::mutations::change_element_orientation_deg::ChangeElementOrientationDeg;
use crate::standards::v1::subsets::any::schema::mutations::change_has_mechanical_ventilation::ChangeHasMechanicalVentilation;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_application_type::ChangeLayerApplicationType;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_compressive_class::ChangeLayerCompressiveClass;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_lambda::ChangeLayerLambda;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_material_id::ChangeLayerMaterialId;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_mu::ChangeLayerMu;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_thickness::ChangeLayerThickness;
use crate::standards::v1::subsets::any::schema::mutations::change_rh_int::ChangeRhInt;
use crate::standards::v1::subsets::any::schema::mutations::change_t_int_c::ChangeTIntC;
use crate::standards::v1::subsets::any::schema::mutations::change_thermal_bridge_bb2_type::ChangeThermalBridgeBb2Type;
use crate::standards::v1::subsets::any::schema::mutations::change_thermal_bridge_length::ChangeThermalBridgeLength;
use crate::standards::v1::subsets::any::schema::mutations::change_thermal_bridge_psi::ChangeThermalBridgePsi;
use crate::standards::v1::subsets::any::schema::mutations::change_usage::ChangeUsage;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_floor_area::ChangeZoneFloorArea;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_heaviness::ChangeZoneHeaviness;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_night_ventilation::ChangeZoneNightVentilation;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_window_area::ChangeZoneWindowArea;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_window_g_value::ChangeZoneWindowGValue;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_window_inclination_deg::ChangeZoneWindowInclinationDeg;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_window_orientation::ChangeZoneWindowOrientation;
use crate::standards::v1::subsets::any::schema::mutations::change_zone_window_shading_fc::ChangeZoneWindowShadingFc;
use crate::standards::v1::subsets::any::schema::mutations::insert_element::InsertElement;
use crate::standards::v1::subsets::any::schema::mutations::insert_layer::InsertLayer;
use crate::standards::v1::subsets::any::schema::mutations::insert_thermal_bridge::InsertThermalBridge;
use crate::standards::v1::subsets::any::schema::mutations::insert_zone::InsertZone;
use crate::standards::v1::subsets::any::schema::mutations::insert_zone_window::InsertZoneWindow;
use crate::standards::v1::subsets::any::schema::mutations::remove_element::RemoveElement;
use crate::standards::v1::subsets::any::schema::mutations::remove_layer::RemoveLayer;
use crate::standards::v1::subsets::any::schema::mutations::remove_thermal_bridge::RemoveThermalBridge;
use crate::standards::v1::subsets::any::schema::mutations::remove_zone::RemoveZone;
use crate::standards::v1::subsets::any::schema::mutations::remove_zone_window::RemoveZoneWindow;
use crate::standards::v1::subsets::any::schema::mutations::reorder_layers::ReorderLayers;
use crate::{Din4108Snapshot, EnvelopeElement, LayerDocument, ThermalBridge, ThermalZone, ZoneWindow};
use std::path::PathBuf;

fn fixtures_root_real() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🎫️fixtures/🧬️mutations")
}

fn folder_for_kind(kind: &str) -> &'static str {
    match kind {
        "change-climate-zone" => "🌦️change-climate-zone",
        "change-usage" => "🗂️change-usage",
        "change-t-int-c" => "🌡️change-t-int-c",
        "change-rh-int" => "💧️change-rh-int",
        "change-airtightness-n50" => "💨️change-airtightness-n50",
        "change-has-mechanical-ventilation" => "🌬️change-has-mechanical-ventilation",
        "change-bb2-details-conform" => "✅️change-bb2-details-conform",
        "insert-zone" => "➕️insert-zone",
        "remove-zone" => "➖️remove-zone",
        "change-zone-floor-area" => "📐️change-zone-floor-area",
        "change-zone-heaviness" => "🧱change-zone-heaviness",
        "change-zone-night-ventilation" => "🌙change-zone-night-ventilation",
        "insert-zone-window" => "🪟insert-zone-window",
        "remove-zone-window" => "🚫️remove-zone-window",
        "change-zone-window-area" => "📏change-zone-window-area",
        "change-zone-window-g-value" => "☀️change-zone-window-g-value",
        "change-zone-window-shading-fc" => "⛱️change-zone-window-shading-fc",
        "insert-element" => "🏠️insert-element",
        "remove-element" => "🚫️remove-element",
        "change-element-area" => "📐️change-element-area",
        "change-element-adjacent" => "↔️change-element-adjacent",
        "change-element-kind" => "🏷️change-element-kind",
        "insert-layer" => "➕️insert-layer",
        "remove-layer" => "➖️remove-layer",
        "reorder-layers" => "🔀️reorder-layers",
        "change-layer-thickness" => "📏️change-layer-thickness",
        "change-layer-lambda" => "🌡change-layer-lambda",
        "change-layer-mu" => "💧change-layer-mu",
        "change-layer-material-id" => "🧽️change-layer-material-id",
        "insert-thermal-bridge" => "🌉️insert-thermal-bridge",
        "remove-thermal-bridge" => "🧊remove-thermal-bridge",
        "change-thermal-bridge-psi" => "🔘change-thermal-bridge-psi",
        "change-thermal-bridge-length" => "↔️change-thermal-bridge-length",
        "change-element-orientation-deg" => "🧭change-element-orientation-deg",
        "change-element-inclination-deg" => "📐change-element-inclination-deg",
        "change-element-delta-ug" => "Δchange-element-delta-ug",
        "change-element-delta-uf" => "Δchange-element-delta-uf",
        "change-element-delta-ur" => "Δchange-element-delta-ur",
        "change-thermal-bridge-bb2-type" => "🏷change-thermal-bridge-bb2-type",
        "change-zone-window-orientation" => "🧭change-zone-window-orientation",
        "change-zone-window-inclination-deg" => "📐change-zone-window-inclination-deg",
        "change-layer-application-type" => "🏷️change-layer-application-type",
        "change-layer-compressive-class" => "🏷️change-layer-compressive-class",
        _ => "unknown",
    }
}

fn sample_mutation(base: &Din4108Snapshot, kind: &str) -> Din4108Mutation {
    let e0 = base.elements[0].id.clone();
    let z0 = base.zones[0].id.clone();
    let w0 = base.zones[0].windows[0].id.clone();
    let b0 = base.thermal_bridges[0].id.clone();
    match kind {
        "change-climate-zone" => Din4108Mutation::ChangeClimateZone(ChangeClimateZone { new_climate_zone: crate::document::ClimateZoneDe::Zone3 }),
        "change-usage" => Din4108Mutation::ChangeUsage(ChangeUsage { new_usage: "nonResidential".into() }),
        "change-t-int-c" => Din4108Mutation::ChangeTIntC(ChangeTIntC { new_t_int_c: 22.0 }),
        "change-rh-int" => Din4108Mutation::ChangeRhInt(ChangeRhInt { new_rh_int: 0.55 }),
        "change-airtightness-n50" => Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 2.0 }),
        "change-has-mechanical-ventilation" => Din4108Mutation::ChangeHasMechanicalVentilation(ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: false }),
        "change-bb2-details-conform" => Din4108Mutation::ChangeBb2DetailsConform(ChangeBb2DetailsConform { new_bb2_details_conform: false }),
        "insert-zone" => Din4108Mutation::InsertZone(InsertZone { index: base.zones.len(), zone: ThermalZone { id: "zone-extra".into(), floor_area_m2: 40.0, heaviness: "medium".into(), night_ventilation: "none".into(), windows: vec![] } }),
        "remove-zone" => Din4108Mutation::RemoveZone(RemoveZone { index: 0 }),
        "change-zone-floor-area" => Din4108Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id: z0, new_floor_area_m2: 90.0 }),
        "change-zone-heaviness" => Din4108Mutation::ChangeZoneHeaviness(ChangeZoneHeaviness { zone_id: z0, new_heaviness: "light".into() }),
        "change-zone-night-ventilation" => Din4108Mutation::ChangeZoneNightVentilation(ChangeZoneNightVentilation { zone_id: z0, new_night_ventilation: "high".into() }),
        "insert-zone-window" => Din4108Mutation::InsertZoneWindow(InsertZoneWindow { zone_id: z0, index: 0, window: ZoneWindow { id: "win-extra".into(), orientation: "W".into(), inclination_deg: 90.0, area_m2: 2.0, g_value: 0.5, shading_fc: 0.5 } }),
        "remove-zone-window" => Din4108Mutation::RemoveZoneWindow(RemoveZoneWindow { zone_id: z0, index: 0 }),
        "change-zone-window-area" => Din4108Mutation::ChangeZoneWindowArea(ChangeZoneWindowArea { zone_id: z0, window_id: w0, new_area_m2: 10.0 }),
        "change-zone-window-g-value" => Din4108Mutation::ChangeZoneWindowGValue(ChangeZoneWindowGValue { zone_id: z0, window_id: w0, new_g_value: 0.6 }),
        "change-zone-window-shading-fc" => Din4108Mutation::ChangeZoneWindowShadingFc(ChangeZoneWindowShadingFc { zone_id: z0, window_id: w0, new_shading_fc: 0.3 }),
        "insert-element" => Din4108Mutation::InsertElement(InsertElement { index: base.elements.len(), element: EnvelopeElement { id: "wall-extra".into(), kind: "wall".into(), zone_id: z0, orientation_deg: 90.0, inclination_deg: 90.0, adjacent: "exterior".into(), area_m2: 10.0, delta_u_g: 0.0, delta_u_f: 0.0, delta_u_r: 0.0, layers: vec![] } }),
        "remove-element" => Din4108Mutation::RemoveElement(RemoveElement { index: 0 }),
        "change-element-area" => Din4108Mutation::ChangeElementArea(ChangeElementArea { element_id: e0, new_area_m2: 45.0 }),
        "change-element-adjacent" => Din4108Mutation::ChangeElementAdjacent(ChangeElementAdjacent { element_id: e0, new_adjacent: "unheated".into() }),
        "change-element-kind" => Din4108Mutation::ChangeElementKind(ChangeElementKind { element_id: e0, new_kind: "frameOpaque".into() }),
        "insert-layer" => Din4108Mutation::InsertLayer(InsertLayer { element_id: e0, index: 0, layer: LayerDocument { id: "layer-extra".into(), material_id: "eps".into(), thickness_m: 0.05, lambda: 0.035, mu: 40.0, density: 20.0, application_type: "WAP".into(), compressive_class: "dm".into(), water_class: "wf".into(), tensile_class: "tf".into(), acoustic_class: "sm".into(), segments: vec![] } }),
        "remove-layer" => Din4108Mutation::RemoveLayer(RemoveLayer { element_id: e0, index: 0 }),
        "reorder-layers" => Din4108Mutation::ReorderLayers(ReorderLayers { element_id: e0, from: 0, to: 1 }),
        "change-layer-thickness" => Din4108Mutation::ChangeLayerThickness(ChangeLayerThickness { element_id: e0, index: 2, new_thickness_m: 0.16 }),
        "change-layer-lambda" => Din4108Mutation::ChangeLayerLambda(ChangeLayerLambda { element_id: e0, index: 2, new_lambda: 0.04 }),
        "change-layer-mu" => Din4108Mutation::ChangeLayerMu(ChangeLayerMu { element_id: e0, index: 2, new_mu: 50.0 }),
        "change-layer-material-id" => Din4108Mutation::ChangeLayerMaterialId(ChangeLayerMaterialId { element_id: e0, index: 2, new_material_id: "xps".into() }),
        "insert-thermal-bridge" => Din4108Mutation::InsertThermalBridge(InsertThermalBridge { index: base.thermal_bridges.len(), bridge: ThermalBridge { id: "tb-extra".into(), psi: 0.08, length_m: 5.0, bb2_type: "categoryB".into() } }),
        "remove-thermal-bridge" => Din4108Mutation::RemoveThermalBridge(RemoveThermalBridge { index: 0 }),
        "change-thermal-bridge-psi" => Din4108Mutation::ChangeThermalBridgePsi(ChangeThermalBridgePsi { bridge_id: b0, new_psi: 0.08 }),
        "change-thermal-bridge-length" => Din4108Mutation::ChangeThermalBridgeLength(ChangeThermalBridgeLength { bridge_id: b0, new_length_m: 30.0 }),
        "change-element-orientation-deg" => Din4108Mutation::ChangeElementOrientationDeg(ChangeElementOrientationDeg { element_id: e0, new_orientation_deg: 180.0 }),
        "change-element-inclination-deg" => Din4108Mutation::ChangeElementInclinationDeg(ChangeElementInclinationDeg { element_id: e0, new_inclination_deg: 45.0 }),
        "change-element-delta-ug" => Din4108Mutation::ChangeElementDeltaUg(ChangeElementDeltaUg { element_id: e0, new_delta_u_g: 0.02 }),
        "change-element-delta-uf" => Din4108Mutation::ChangeElementDeltaUf(ChangeElementDeltaUf { element_id: e0, new_delta_u_f: 0.01 }),
        "change-element-delta-ur" => Din4108Mutation::ChangeElementDeltaUr(ChangeElementDeltaUr { element_id: e0, new_delta_u_r: 0.005 }),
        "change-thermal-bridge-bb2-type" => Din4108Mutation::ChangeThermalBridgeBb2Type(ChangeThermalBridgeBb2Type { bridge_id: b0, new_bb2_type: "categoryB".into() }),
        "change-zone-window-orientation" => Din4108Mutation::ChangeZoneWindowOrientation(ChangeZoneWindowOrientation { zone_id: z0, window_id: w0, new_orientation: "N".into() }),
        "change-zone-window-inclination-deg" => Din4108Mutation::ChangeZoneWindowInclinationDeg(ChangeZoneWindowInclinationDeg { zone_id: z0, window_id: w0, new_inclination_deg: 60.0 }),
        "change-layer-application-type" => Din4108Mutation::ChangeLayerApplicationType(ChangeLayerApplicationType { element_id: e0, index: 2, new_application_type: "WAB".into() }),
        "change-layer-compressive-class" => Din4108Mutation::ChangeLayerCompressiveClass(ChangeLayerCompressiveClass { element_id: e0, index: 2, new_compressive_class: "dk".into() }),
        other => panic!("unhandled kind {other}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn mutation_fixture_module_compiles() {
    let base = Din4108Snapshot::compliant_etics_dwelling();
    let root = fixtures_root_real();
    std::fs::create_dir_all(&root).ok();
    for kind in KINDS {
        let folder = folder_for_kind(kind);
        assert_ne!(folder, "unknown", "folder map missing for {kind}");
        let case = root.join(folder).join(format!("🧪sets-{kind}"));
        std::fs::create_dir_all(case.join("📸️snapshot/⬅️before")).unwrap();
        std::fs::create_dir_all(case.join("📸️snapshot/➡️after")).unwrap();
        std::fs::create_dir_all(case.join("🦠️mutation")).unwrap();
        std::fs::create_dir_all(case.join("🎯️outcome")).unwrap();
        std::fs::create_dir_all(case.join("🔺️diff")).unwrap();
        let mutation = sample_mutation(&base, kind);
        let (after, _) = apply_din4108_mutation(&base, &mutation).expect("apply");
        assert_ne!(base, after, "{kind} must change snapshot");
        let before_json = serde_json::to_string_pretty(&base).unwrap();
        let after_json = serde_json::to_string_pretty(&after).unwrap();
        let mut_json = serde_json::to_string_pretty(&mutation).unwrap();
        std::fs::write(case.join("📸️snapshot/⬅️before/🔣️.json"), before_json).unwrap();
        std::fs::write(case.join("📸️snapshot/➡️after/🔣️.json"), after_json).unwrap();
        std::fs::write(case.join("🦠️mutation/🔣️.json"), mut_json).unwrap();
        std::fs::write(case.join("🎯️outcome/🔣️.json"), "{\"status\":\"applied\"}\n").unwrap();
        std::fs::write(case.join("🔺️diff/🔣️.json"), "{}\n").unwrap();
    }
}
