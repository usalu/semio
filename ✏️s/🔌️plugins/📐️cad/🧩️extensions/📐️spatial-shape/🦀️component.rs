//! 🧩️ CAD spatial-shape extension — contributes shape stat/property computers to `cad-play`.

use semio_framework_core::Contribution;
use semio_framework_plugin::ExtensionBundle;
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

fn computers_manifest() -> CadComputersManifest {
    CadComputersManifest {
        model_definition_ids: vec!["spatial.shape"],
        stat_computers: vec!["spatial.shape.geometry"],
        property_computers: vec!["spatial.shape.volume"],
        import_profiles: Vec::new(),
        transformation_appliers: Vec::new(),
    }
}

fn bundle() -> ExtensionBundle {
    ExtensionBundle::new(EXTENSION_ID, "CAD Spatial Shape", "0.1.0")
        .extends("cad")
        .contributes(Contribution::CadComputer {
            app_id: HOST_APP_ID.into(),
            module_id: MODULE_ID.into(),
            label: "Spatial Shape".into(),
            icon_id: "box".into(),
            computers_json: serde_json::to_string(&computers_manifest()).unwrap_or_default(),
        })
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_contributes_spatial_shape_for_cad_play() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extends, "cad");
        assert_eq!(manifest.contributions.len(), 1);
        let Contribution::CadComputer { app_id, module_id, computers_json, .. } = &manifest.contributions[0] else {
            panic!("expected CadComputer");
        };
        assert_eq!(app_id, HOST_APP_ID);
        assert_eq!(module_id, MODULE_ID);
        let parsed: serde_json::Value = serde_json::from_str(computers_json).expect("computers_json");
        assert_eq!(parsed["statComputers"], serde_json::json!(["spatial.shape.geometry"]));
    }
}
//#endregion 🧪️Tests
