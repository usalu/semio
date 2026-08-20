//! 🧩️ CAD aec-building-structure extension — contributes structure computers, transforms, and STEP import to `cad-play`.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use serde::Serialize;
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building-structure";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building-structure";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CadImportProfileManifest {
    model_definition_id: &'static str,
    layer_typology: BTreeMap<&'static str, &'static str>,
    fallback_typology: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_presentation_layers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presentation_geometry: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_domain: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CadComputersManifest {
    model_definition_ids: Vec<&'static str>,
    stat_computers: Vec<&'static str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    property_computers: Vec<&'static str>,
    import_profiles: Vec<CadImportProfileManifest>,
    transformation_appliers: Vec<&'static str>,
}

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

fn structure_import_profile(model_definition_id: &'static str, prefer_presentation_layers: bool, presentation_geometry: Option<&'static str>) -> CadImportProfileManifest {
    CadImportProfileManifest {
        model_definition_id,
        layer_typology: structure_layer_typology(),
        fallback_typology: "structure.structure.onewayreinforcedconcreteslab",
        prefer_presentation_layers: if prefer_presentation_layers { Some(true) } else { None },
        presentation_geometry,
        namespaced_domain: Some("structure"),
    }
}

fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec![
            "aec.building.structure",
            "aec.building.structure.classic",
            "aec.building.structure.fem.line",
            "aec.building.structure.fem.solid",
            "aec.building.structure.fem.surface",
        ],
        stat_computers: vec!["structure.stability"],
        property_computers: Vec::new(),
        import_profiles: vec![
            structure_import_profile("aec.building.structure", false, None),
            structure_import_profile("aec.building.structure.classic", true, Some("wireframe")),
            structure_import_profile("aec.building.structure.fem.line", false, None),
            structure_import_profile("aec.building.structure.fem.solid", false, None),
            structure_import_profile("aec.building.structure.fem.surface", false, None),
        ],
        transformation_appliers: vec!["aec.building.structure/from_building"],
    }
}

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let bundle = ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building Structure", "0.1.0").extends("cad");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`cad.computer`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "cad.computer",
        serde_json::json!({
            "appId": HOST_APP_ID,
            "moduleId": MODULE_ID,
            "label": "AEC Building Structure",
            "iconId": "landmark",
            "computersJson": serde_json::to_string(&computers_manifest()).unwrap_or_default(),
        }),
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
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("parse");
        assert_eq!(parsed["importProfiles"].as_array().map(|rows| rows.len()), Some(5));
        assert_eq!(parsed["transformationAppliers"], serde_json::json!(["aec.building.structure/from_building"]));
    }
}
//#endregion 🧪️Tests
