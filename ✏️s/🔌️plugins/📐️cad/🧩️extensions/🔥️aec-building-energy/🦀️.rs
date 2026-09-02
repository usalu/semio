//! 🧩️ CAD aec-building-energy extension — contributes energy computers and STEP import to `cad-play`.
//!
//! `computersJson` is built with `pack::json` (first-party `serde_json::Value` replacement)
//! instead of a `#[derive(ToValue)]` DTO. `contributes_topic`/`TopicContribution` now speak
//! `semio_framework::DslValue` end to end (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s `TopicContribution`
//! seam), so `serde`/`serde_json` are fully gone from this crate.

use pack::json::{self, Value as JsonValue};
use semio_framework::DslValue;
use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building-energy";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building-energy";

fn energy_layer_typology() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("slab", "energy.energy.baseplate"),
        ("baseplate", "energy.energy.baseplate"),
        ("roof", "energy.energy.roof"),
        ("wall", "energy.energy.externalwall"),
        ("walls", "energy.energy.externalwall"),
        ("hull", "energy.energy.hull"),
        ("window", "energy.energy.windows"),
        ("windows", "energy.energy.windows"),
    ])
}

/// 🗂️ `pack::json` analog of the former `CadImportProfileManifest`.
fn energy_import_profile(model_definition_id: &'static str, prefer_presentation_layers: bool, presentation_geometry: Option<&'static str>) -> JsonValue {
    let mut entries: Vec<(String, JsonValue)> = vec![
        ("modelDefinitionId".to_string(), JsonValue::from(model_definition_id)),
        ("layerTypology".to_string(), json::object(energy_layer_typology().into_iter().map(|(key, value)| (key.to_string(), JsonValue::from(value))))),
        ("fallbackTypology".to_string(), JsonValue::from("energy.energy.hull")),
    ];
    if prefer_presentation_layers {
        entries.push(("preferPresentationLayers".to_string(), JsonValue::from(true)));
    }
    if let Some(geometry) = presentation_geometry {
        entries.push(("presentationGeometry".to_string(), JsonValue::from(geometry)));
    }
    entries.push(("namespacedDomain".to_string(), JsonValue::from("energy")));
    json::object(entries)
}

/// 🗂️ `pack::json` analog of the former `CadComputersManifest`.
fn computers_manifest() -> JsonValue {
    json::object([
        ("modelDefinitionIds".to_string(), json::array([JsonValue::from("aec.building.energy")])),
        ("statComputers".to_string(), json::array([JsonValue::from("energy.demand")])),
        ("propertyComputers".to_string(), json::array([JsonValue::from("energy.heatedvolume")])),
        ("importProfiles".to_string(), json::array([energy_import_profile("aec.building.energy", true, Some("wireframe"))])),
        ("transformationAppliers".to_string(), json::array([])),
    ])
}

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let bundle = ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building Energy", "0.1.0").extends("cad");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`cad.computer`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "cad.computer",
        DslValue::object([
            ("appId".to_string(), DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), DslValue::String(MODULE_ID.to_string())),
            ("label".to_string(), DslValue::String("AEC Building Energy".to_string())),
            ("iconId".to_string(), DslValue::String("zap".to_string())),
            ("computersJson".to_string(), DslValue::String(json::to_string(&computers_manifest()))),
        ]),
    ))
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn bundle_contributes_energy_computers() {
        let manifest = bundle().manifest;
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed = json::parse(computers_json).expect("parse");
        assert_eq!(parsed.get("statComputers"), Some(&json::array([JsonValue::from("energy.demand")])));
        assert_eq!(parsed.get("propertyComputers"), Some(&json::array([JsonValue::from("energy.heatedvolume")])));
    }
}
//#endregion 🧪️Tests
