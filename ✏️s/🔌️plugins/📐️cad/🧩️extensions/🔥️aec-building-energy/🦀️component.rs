//! 🧩️ CAD aec-building-energy extension — contributes energy computers and STEP import to `cad-play`.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use serde::Serialize;
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building-energy";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building-energy";

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
    property_computers: Vec<&'static str>,
    import_profiles: Vec<CadImportProfileManifest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    transformation_appliers: Vec<&'static str>,
}

async fn energy_layer_typology() -> BTreeMap<&'static str, &'static str> {
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

async fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec!["aec.building.energy"],
        stat_computers: vec!["energy.demand"],
        property_computers: vec!["energy.heatedvolume"],
        import_profiles: vec![CadImportProfileManifest {
            model_definition_id: "aec.building.energy",
            layer_typology: energy_layer_typology(),
            fallback_typology: "energy.energy.hull",
            prefer_presentation_layers: Some(true),
            presentation_geometry: Some("wireframe"),
            namespaced_domain: Some("energy"),
        }],
        transformation_appliers: Vec::new(),
    }
}

async fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building Energy", "0.1.0")
        .extends("cad")
        // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
        // extension only contributes a topic (`cad.computer`).
        .mode(ExecutionMode::Declarative)
        .contributes_topic(
            "cad.computer",
            serde_json::json!({
                "appId": HOST_APP_ID,
                "moduleId": MODULE_ID,
                "label": "AEC Building Energy",
                "iconId": "zap",
                "computersJson": serde_json::to_string(&computers_manifest()).unwrap_or_default(),
            }),
        )
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn bundle_contributes_energy_computers() {
        let manifest = bundle().manifest;
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("parse");
        assert_eq!(parsed["statComputers"], serde_json::json!(["energy.demand"]));
        assert_eq!(parsed["propertyComputers"], serde_json::json!(["energy.heatedvolume"]));
    }
}
//#endregion 🧪️Tests
