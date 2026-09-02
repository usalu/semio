//! 🧩️ CAD spatial-shape extension — contributes shape stat/property computers to `cad-play`.
//!
//! `computersJson` is built with `pack::json` (first-party `serde_json::Value` replacement)
//! instead of a `#[derive(ToValue)]` DTO — `serde`/`serde_json` are fully gone from this crate,
//! including at the `bundle.contributes_topic(...)` call, which now speaks
//! `semio_framework_os_kernel::DslValue` end to end (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s `TopicContribution` seam).

use pack::json;
use semio_framework::DslValue;
use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

//#region 🔖️Manifest
const EXTENSION_ID: &str = "cad-extension-spatial-shape";
const HOST_APP_ID: &str = "cad-play";
const MODULE_ID: &str = "spatial-shape";

fn computers_manifest() -> json::Value {
    json::object([
        ("modelDefinitionIds".to_string(), json::array([json::Value::from("spatial.shape")])),
        ("statComputers".to_string(), json::array([json::Value::from("spatial.shape.geometry")])),
        ("propertyComputers".to_string(), json::array([json::Value::from("spatial.shape.volume")])),
        ("importProfiles".to_string(), json::array([])),
        ("transformationAppliers".to_string(), json::array([])),
    ])
}

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let bundle = ExtensionBundle::new(EXTENSION_ID, "CAD Spatial Shape", "0.1.0").extends("cad");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`cad.computer`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "cad.computer",
        DslValue::object([
            ("appId".to_string(), DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), DslValue::String(MODULE_ID.to_string())),
            ("label".to_string(), DslValue::String("Spatial Shape".to_string())),
            ("iconId".to_string(), DslValue::String("box".to_string())),
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
    async fn bundle_contributes_spatial_shape_for_cad_play() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extends, "cad");
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic_contribution = &manifest.topic_contributions[0];
        assert_eq!(topic_contribution.topic, "cad.computer");
        assert_eq!(topic_contribution.payload["appId"].as_str(), Some(HOST_APP_ID));
        assert_eq!(topic_contribution.payload["moduleId"].as_str(), Some(MODULE_ID));
        let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
        let parsed = json::parse(computers_json).expect("computers_json");
        assert_eq!(parsed.get("statComputers"), Some(&json::array([json::Value::from("spatial.shape.geometry")])));
    }
}
//#endregion 🧪️Tests
