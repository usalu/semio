//! 🧩️ CAD aec-building extension — contributes building STEP import profile to `cad-play`.

use semio_framework_plugin::ExtensionBundle;
use serde::Serialize;
use std::collections::BTreeMap;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-aec-building";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "aec-building";

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    stat_computers: Vec<&'static str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    property_computers: Vec<&'static str>,
    import_profiles: Vec<CadImportProfileManifest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    transformation_appliers: Vec<&'static str>,
}

fn building_layer_typology() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("slab", "building.building.slab"),
        ("slabs", "building.building.slab"),
        ("beam", "building.building.beam"),
        ("beams", "building.building.beam"),
        ("column", "building.building.column"),
        ("columns", "building.building.column"),
        ("wall", "building.building.wall"),
        ("walls", "building.building.wall"),
        ("roof", "building.building.roof"),
        ("roofs", "building.building.roof"),
        ("foundation", "building.building.foundation"),
        ("foundations", "building.building.foundation"),
        ("stair", "building.building.stair"),
        ("stairs", "building.building.stair"),
        ("ceiling", "building.building.ceiling"),
        ("ceilings", "building.building.ceiling"),
        ("railing", "building.building.railing"),
        ("railings", "building.building.railing"),
        ("door", "building.building.door"),
        ("doors", "building.building.door"),
        ("window", "building.building.window"),
        ("windows", "building.building.window"),
    ])
}

fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec!["aec.building"],
        stat_computers: Vec::new(),
        property_computers: Vec::new(),
        import_profiles: vec![CadImportProfileManifest {
            model_definition_id: "aec.building",
            layer_typology: building_layer_typology(),
            fallback_typology: "building.building.slab",
            prefer_presentation_layers: None,
            presentation_geometry: None,
            namespaced_domain: None,
        }],
        transformation_appliers: Vec::new(),
    }
}

fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", "0.1.0")
        .extends("cad")
        .contributes_topic(
            "cad.computer",
            serde_json::json!({
                "appId": HOST_APP_ID,
                "moduleId": MODULE_ID,
                "label": "AEC Building",
                "iconId": "building",
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
    fn bundle_contributes_building_import_profile() {
        let manifest = bundle().manifest;
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        assert_eq!(topic_contribution.payload["moduleId"], MODULE_ID);
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("parse");
        assert!(parsed["importProfiles"][0]["layerTypology"]["beam"].as_str().is_some());
    }
}
//#endregion 🧪️Tests
