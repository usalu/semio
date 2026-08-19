//! 🧩️ CAD spatial-shape extension — contributes shape stat/property computers to `cad-play`.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use serde::Serialize;

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-spatial-shape";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "spatial-shape";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CadComputersManifest {
    model_definition_ids: Vec<&'static str>,
    stat_computers: Vec<&'static str>,
    property_computers: Vec<&'static str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    import_profiles: Vec<()>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    transformation_appliers: Vec<&'static str>,
}

async fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec!["spatial.shape"],
        stat_computers: vec!["spatial.shape.geometry"],
        property_computers: vec!["spatial.shape.volume"],
        import_profiles: Vec::new(),
        transformation_appliers: Vec::new(),
    }
}

async fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD Spatial Shape", "0.1.0")
        .extends("cad")
        // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
        // extension only contributes a topic (`cad.computer`).
        .mode(ExecutionMode::Declarative)
        .contributes_topic(
            "cad.computer",
            serde_json::json!({
                "appId": HOST_APP_ID,
                "moduleId": MODULE_ID,
                "label": "Spatial Shape",
                "iconId": "box",
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
    async fn bundle_contributes_spatial_shape_for_cad_play() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extends, "cad");
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        assert_eq!(topic_contribution.payload["appId"], HOST_APP_ID);
        assert_eq!(topic_contribution.payload["moduleId"], MODULE_ID);
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("computers_json");
        assert_eq!(parsed["statComputers"], serde_json::json!(["spatial.shape.geometry"]));
    }
}
//#endregion 🧪️Tests
