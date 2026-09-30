"""🧪️ W2-W-norm-2: writes the 36 DIN 4108 leaf inverses that returned `Vec::new()` (undo of a
collection-level edit was silently lost). Every inverse is computed from BASE state, and a missing
target yields no step, per the addressing convention the independent Python engine implements."""

import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

ELEMENT = "base.elements.iter().find(|element| element.id == payload.element_id)"
BRIDGE = "base.thermal_bridges.iter().find(|bridge| bridge.id == payload.bridge_id)"
ZONE = "base.zones.iter().find(|zone| zone.id == payload.zone_id)"
WINDOW = f"{ZONE}.and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id))"
LAYER = f"{ELEMENT}.and_then(|element| element.layers.get(payload.index))"

CHANGES = {
    "Δchange-element-delta-uf": ("ChangeElementDeltaUf", ELEMENT, "element", "element_id", "new_delta_u_f", "delta_u_f", False),
    "Δchange-element-delta-ug": ("ChangeElementDeltaUg", ELEMENT, "element", "element_id", "new_delta_u_g", "delta_u_g", False),
    "Δchange-element-delta-ur": ("ChangeElementDeltaUr", ELEMENT, "element", "element_id", "new_delta_u_r", "delta_u_r", False),
    "↔️change-element-adjacent": ("ChangeElementAdjacent", ELEMENT, "element", "element_id", "new_adjacent", "adjacent", True),
    "🏷️change-element-kind": ("ChangeElementKind", ELEMENT, "element", "element_id", "new_kind", "kind", True),
    "📐️change-element-area": ("ChangeElementArea", ELEMENT, "element", "element_id", "new_area_m2", "area_m2", False),
    "📐change-element-inclination-deg": ("ChangeElementInclinationDeg", ELEMENT, "element", "element_id", "new_inclination_deg", "inclination_deg", False),
    "🧭change-element-orientation-deg": ("ChangeElementOrientationDeg", ELEMENT, "element", "element_id", "new_orientation_deg", "orientation_deg", False),
    "↔️change-thermal-bridge-length": ("ChangeThermalBridgeLength", BRIDGE, "bridge", "bridge_id", "new_length_m", "length_m", False),
    "🔘change-thermal-bridge-psi": ("ChangeThermalBridgePsi", BRIDGE, "bridge", "bridge_id", "new_psi", "psi", False),
    "🏷change-thermal-bridge-bb2-type": ("ChangeThermalBridgeBb2Type", BRIDGE, "bridge", "bridge_id", "new_bb2_type", "bb2_type", True),
    "🌙change-zone-night-ventilation": ("ChangeZoneNightVentilation", ZONE, "zone", "zone_id", "new_night_ventilation", "night_ventilation", True),
    "📐️change-zone-floor-area": ("ChangeZoneFloorArea", ZONE, "zone", "zone_id", "new_floor_area_m2", "floor_area_m2", False),
    "🧱change-zone-heaviness": ("ChangeZoneHeaviness", ZONE, "zone", "zone_id", "new_heaviness", "heaviness", True),
    "☀️change-zone-window-g-value": ("ChangeZoneWindowGValue", WINDOW, "window", "zone_id, window_id", "new_g_value", "g_value", False),
    "⛱️change-zone-window-shading-fc": ("ChangeZoneWindowShadingFc", WINDOW, "window", "zone_id, window_id", "new_shading_fc", "shading_fc", False),
    "📏change-zone-window-area": ("ChangeZoneWindowArea", WINDOW, "window", "zone_id, window_id", "new_area_m2", "area_m2", False),
    "📐change-zone-window-inclination-deg": ("ChangeZoneWindowInclinationDeg", WINDOW, "window", "zone_id, window_id", "new_inclination_deg", "inclination_deg", False),
    "🧭change-zone-window-orientation": ("ChangeZoneWindowOrientation", WINDOW, "window", "zone_id, window_id", "new_orientation", "orientation", True),
    "🌡change-layer-lambda": ("ChangeLayerLambda", LAYER, "layer", "element_id, index", "new_lambda", "lambda", False),
    "💧change-layer-mu": ("ChangeLayerMu", LAYER, "layer", "element_id, index", "new_mu", "mu", False),
    "📏️change-layer-thickness": ("ChangeLayerThickness", LAYER, "layer", "element_id, index", "new_thickness_m", "thickness_m", False),
    "🧽️change-layer-material-id": ("ChangeLayerMaterialId", LAYER, "layer", "element_id, index", "new_material_id", "material_id", True),
    "🏷️change-layer-application-type": ("ChangeLayerApplicationType", LAYER, "layer", "element_id, index", "new_application_type", "application_type", True),
    "🏷️change-layer-compressive-class": ("ChangeLayerCompressiveClass", LAYER, "layer", "element_id, index", "new_compressive_class", "compressive_class", True),
}

HEADER = "//! ↩️ `{kind}` inverse — {what}, computed from BASE state; a missing target yields no step.\n\n"


def address(fields):
    return ", ".join(f"{field}: payload.{field}" + ("" if field == "index" else ".clone()") for field in [name.strip() for name in fields.split(",")])


def change(directory, spec):
    struct, finder, binding, fields, new, field, owned = spec
    kind = directory[directory.index("change-"):]
    value = f"{binding}.{field}" + (".clone()" if owned else "")
    body = (
        HEADER.format(kind=kind, what=f"restores the {binding}'s `{field}`")
        + f"use super::{struct};\nuse crate::{{Din4108Mutation, Din4108Snapshot}};\n\n"
        + f"pub fn inverse(payload: &{struct}, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {{\n"
        + f"    {finder}.map(|{binding}| vec![Din4108Mutation::{struct}({struct} {{ {address(fields)}, {new}: {value} }})]).unwrap_or_default()\n"
        + "}\n"
    )
    return body


STRUCTURAL = {
    "➕️insert-zone": ("insert-zone", "removes the inserted zone at its landing position",
        "use super::InsertZone;\nuse crate::mutations::remove_zone::RemoveZone;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &InsertZone, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    vec![Din4108Mutation::RemoveZone(RemoveZone { index: payload.index.min(base.zones.len()) })]\n}\n"),
    "➖️remove-zone": ("remove-zone", "re-inserts the removed zone at its position",
        "use super::RemoveZone;\nuse crate::mutations::insert_zone::InsertZone;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &RemoveZone, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.zones.get(payload.index).map(|zone| vec![Din4108Mutation::InsertZone(InsertZone { index: payload.index, zone: zone.clone() })]).unwrap_or_default()\n}\n"),
    "🪟insert-zone-window": ("insert-zone-window", "removes the inserted window at its landing position",
        "use super::InsertZoneWindow;\nuse crate::mutations::remove_zone_window::RemoveZoneWindow;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &InsertZoneWindow, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Din4108Mutation::RemoveZoneWindow(RemoveZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index.min(zone.windows.len()) })]).unwrap_or_default()\n}\n"),
    "🚫️remove-zone-window": ("remove-zone-window", "re-inserts the removed window at its position",
        "use super::RemoveZoneWindow;\nuse crate::mutations::insert_zone_window::InsertZoneWindow;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &RemoveZoneWindow, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.get(payload.index)).map(|window| vec![Din4108Mutation::InsertZoneWindow(InsertZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index, window: window.clone() })]).unwrap_or_default()\n}\n"),
    "🏠️insert-element": ("insert-element", "removes the inserted element at its landing position",
        "use super::InsertElement;\nuse crate::mutations::remove_element::RemoveElement;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &InsertElement, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    vec![Din4108Mutation::RemoveElement(RemoveElement { index: payload.index.min(base.elements.len()) })]\n}\n"),
    "🚫️remove-element": ("remove-element", "re-inserts the removed element at its position",
        "use super::RemoveElement;\nuse crate::mutations::insert_element::InsertElement;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &RemoveElement, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.elements.get(payload.index).map(|element| vec![Din4108Mutation::InsertElement(InsertElement { index: payload.index, element: element.clone() })]).unwrap_or_default()\n}\n"),
    "➕️insert-layer": ("insert-layer", "removes the inserted layer at its landing position",
        "use super::InsertLayer;\nuse crate::mutations::remove_layer::RemoveLayer;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &InsertLayer, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::RemoveLayer(RemoveLayer { element_id: payload.element_id.clone(), index: payload.index.min(element.layers.len()) })]).unwrap_or_default()\n}\n"),
    "➖️remove-layer": ("remove-layer", "re-inserts the removed layer at its position",
        "use super::RemoveLayer;\nuse crate::mutations::insert_layer::InsertLayer;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &RemoveLayer, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.elements.iter().find(|element| element.id == payload.element_id).and_then(|element| element.layers.get(payload.index)).map(|layer| vec![Din4108Mutation::InsertLayer(InsertLayer { element_id: payload.element_id.clone(), index: payload.index, layer: layer.clone() })]).unwrap_or_default()\n}\n"),
    "🔀️reorder-layers": ("reorder-layers", "moves the layer back to its original position",
        "use super::ReorderLayers;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &ReorderLayers, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.elements.iter().find(|element| element.id == payload.element_id).filter(|element| payload.from < element.layers.len() && payload.to < element.layers.len()).map(|_| vec![Din4108Mutation::ReorderLayers(ReorderLayers { element_id: payload.element_id.clone(), from: payload.to, to: payload.from })]).unwrap_or_default()\n}\n"),
    "🌉️insert-thermal-bridge": ("insert-thermal-bridge", "removes the inserted thermal bridge at its landing position",
        "use super::InsertThermalBridge;\nuse crate::mutations::remove_thermal_bridge::RemoveThermalBridge;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &InsertThermalBridge, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    vec![Din4108Mutation::RemoveThermalBridge(RemoveThermalBridge { index: payload.index.min(base.thermal_bridges.len()) })]\n}\n"),
    "🧊remove-thermal-bridge": ("remove-thermal-bridge", "re-inserts the removed thermal bridge at its position",
        "use super::RemoveThermalBridge;\nuse crate::mutations::insert_thermal_bridge::InsertThermalBridge;\nuse crate::{Din4108Mutation, Din4108Snapshot};\n\n",
        "pub fn inverse(payload: &RemoveThermalBridge, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {\n    base.thermal_bridges.get(payload.index).map(|bridge| vec![Din4108Mutation::InsertThermalBridge(InsertThermalBridge { index: payload.index, bridge: bridge.clone() })]).unwrap_or_default()\n}\n"),
}

if __name__ == "__main__":
    written = 0
    for directory, spec in CHANGES.items():
        with open(f"{ROOT}/{directory}/↩️inverse/🦀️.rs", "w") as handle:
            handle.write(change(directory, spec))
        written += 1
    for directory, (kind, what, uses, body) in STRUCTURAL.items():
        with open(f"{ROOT}/{directory}/↩️inverse/🦀️.rs", "w") as handle:
            handle.write(HEADER.format(kind=kind, what=what) + uses + body)
        written += 1
    print(f"wrote {written} inverses")
