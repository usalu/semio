//! 🧩️ CAD aec-building-structure extension — contributes structure computers, transforms, and STEP import to `cad-play`.
//!
//! The `computersJson` payload is built with `pack::json` (first-party `serde_json::Value`
//! replacement) instead of a `#[derive(ToValue)]` DTO. `contributes_topic`/`TopicContribution`
//! now speak `semio_framework::DslValue` end to end (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s `TopicContribution`
//! seam), so `serde`/`serde_json` are fully gone from this crate.

use pack::json::{self, Value as JsonValue};
use semio_framework::DslValue;
use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building-structure";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building-structure";

fn structure_layer_typology() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("slab", "structure.structure.onewayreinforcedconcreteslab"),
        ("column", "structure.structure.reinforcedconcretecolumn"),
        ("columns", "structure.structure.reinforcedconcretecolumn"),
        ("beam", "structure.structure.reinforcedconcreteinternalwall"),
        ("beams", "structure.structure.reinforcedconcreteinternalwall"),
        ("wall", "structure.structure.reinforcedconcreteexternalwall"),
        ("walls", "structure.structure.reinforcedconcreteexternalwall"),
    ])
}

/// 🗂️ `pack::json` analog of the former `CadImportProfileManifest` — one import-profile entry.
fn structure_import_profile(model_definition_id: &'static str, prefer_presentation_layers: bool, presentation_geometry: Option<&'static str>) -> JsonValue {
    let mut entries: Vec<(String, JsonValue)> = vec![
        ("modelDefinitionId".to_string(), JsonValue::from(model_definition_id)),
        ("layerTypology".to_string(), json::object(structure_layer_typology().into_iter().map(|(key, value)| (key.to_string(), JsonValue::from(value))))),
        ("fallbackTypology".to_string(), JsonValue::from("structure.structure.onewayreinforcedconcreteslab")),
    ];
    if prefer_presentation_layers {
        entries.push(("preferPresentationLayers".to_string(), JsonValue::from(true)));
    }
    if let Some(geometry) = presentation_geometry {
        entries.push(("presentationGeometry".to_string(), JsonValue::from(geometry)));
    }
    entries.push(("namespacedDomain".to_string(), JsonValue::from("structure")));
    json::object(entries)
}

/// 🗂️ `pack::json` analog of the former `CadComputersManifest`.
fn computers_manifest() -> JsonValue {
    json::object([
        ("modelDefinitionIds".to_string(), json::array(["aec.building.structure", "aec.building.structure.classic", "aec.building.structure.fem.line", "aec.building.structure.fem.solid", "aec.building.structure.fem.surface"].map(JsonValue::from))),
        ("statComputers".to_string(), json::array([JsonValue::from("structure.stability")])),
        ("propertyComputers".to_string(), json::array([])),
        (
            "importProfiles".to_string(),
            json::array([
                structure_import_profile("aec.building.structure", false, None),
                structure_import_profile("aec.building.structure.classic", true, Some("wireframe")),
                structure_import_profile("aec.building.structure.fem.line", false, None),
                structure_import_profile("aec.building.structure.fem.solid", false, None),
                structure_import_profile("aec.building.structure.fem.surface", false, None),
            ]),
        ),
        ("transformationAppliers".to_string(), json::array([JsonValue::from("aec.building.structure/from_building")])),
    ])
}

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let bundle = ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building Structure", "0.1.0").extends("cad");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`cad.computer`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "cad.computer",
        DslValue::object([
            ("appId".to_string(), DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), DslValue::String(MODULE_ID.to_string())),
            ("label".to_string(), DslValue::String("AEC Building Structure".to_string())),
            ("iconId".to_string(), DslValue::String("landmark".to_string())),
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
    async fn bundle_contributes_structure_manifest() {
        let manifest = bundle().manifest;
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed = json::parse(computers_json).expect("parse");
        assert_eq!(parsed.get("importProfiles").and_then(JsonValue::as_array).map(|rows| rows.len()), Some(5));
        assert_eq!(parsed.get("transformationAppliers"), Some(&json::array([JsonValue::from("aec.building.structure/from_building")])));
    }
}
//#endregion 🧪️Tests
