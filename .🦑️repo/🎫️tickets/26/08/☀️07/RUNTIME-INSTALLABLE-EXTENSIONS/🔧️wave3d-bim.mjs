import fs from "fs";

const path = fs.readFileSync(new URL("./bim-component.path", import.meta.url), "utf8").trim();
let src = fs.readFileSync(path, "utf8");

const oldCfg = `#[cfg(any(test, target_arch = "wasm32", feature = "component-guest"))]
fn module_registry() -> Registry {`;
const newCfg = `#[cfg(any(test, feature = "component-guest"))]
fn module_registry() -> Registry {`;
if (!src.includes(oldCfg)) {
  console.error("module_registry cfg not found");
  process.exit(1);
}
src = src.replace(oldCfg, newCfg);

const oldGuestAndWasm = `// #region 🔖️PluginGuest
#[cfg(feature = "component-guest")]
mod plugin_guest {
    use super::module_registry;
    use flow_extension_sdk::build_manifest_json;
    use semio_framework_core::Contribution;
    use semio_framework_plugin::PluginBundle;

    const PLUGIN_ID: &str = "flow-extension-bim";
    const HOST_APP_ID: &str = "procedural3d-play";

    fn bundle() -> PluginBundle {
        let manifest_json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        PluginBundle::new(PLUGIN_ID, "Flow Extension Bim", "0.1.0").contributes(Contribution::FlowExtension {
            app_id: HOST_APP_ID.into(),
            extension_id: "bim".into(),
            label: "Bim".into(),
            icon_id: "bim".into(),
            manifest_json,
        })
    }

    semio_framework_plugin::plugin_exports!(bundle);
}
// #endregion 🔖️PluginGuest

// #region 🔖️WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, command_json, evaluate_json};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖️WasmExt`;

const newGuest = `// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_json};
    use semio_framework_core::{Contribution, Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::ExtensionBundle;
    use serde::Deserialize;

    const HOST_APP_ID: &str = "procedural3d-play";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        ExtensionBundle::new("bim", "Bim", "0.1.0")
            .extends("flow")
            .contributes(Contribution::FlowExtension {
                app_id: HOST_APP_ID.into(),
                extension_id: "bim".into(),
                label: "Bim".into(),
                icon_id: "bim".into(),
                manifest_json,
            })
            .handler("evaluate", |req| {
                let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string())
                })?;
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            })
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest`;

if (!src.includes(oldGuestAndWasm)) {
  console.error("PluginGuest/WasmExt block not found");
  console.error(src.slice(-2500));
  process.exit(1);
}
src = src.replace(oldGuestAndWasm, newGuest);

const insertBefore = `}
// #endregion 🔖️Tests`;

const bundleTest = `
    #[test]
    fn extension_bundle_extends_flow_and_evaluates() {
        use semio_framework_core::Contribution;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let manifest_json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        let bundle = ExtensionBundle::new("bim", "Bim", "0.1.0")
            .extends("flow")
            .contributes(Contribution::FlowExtension {
                app_id: "procedural3d-play".into(),
                extension_id: "bim".into(),
                label: "Bim".into(),
                icon_id: "bim".into(),
                manifest_json,
            })
            .handler("evaluate", |req| {
                #[derive(serde::Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct EvaluateRequest {
                    operator_id: String,
                    input_json: String,
                }
                let request: EvaluateRequest = serde_json::from_slice(req).unwrap();
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            });
        install_extension_bundle(bundle);
        extension_activate().unwrap();
        let installed = extension_manifest();
        assert_eq!(installed.extension_id, "bim");
        assert_eq!(installed.extends, "flow");
        assert!(matches!(installed.contributions[0], Contribution::FlowExtension { .. }));
        let input = Dictionary::new()
            .insert("length", Value::Dictionary(number_dictionary(4.0)))
            .insert("height", Value::Dictionary(number_dictionary(2.8)))
            .insert("thickness", Value::Dictionary(number_dictionary(0.2)));
        let req = serde_json::json!({
            "operatorId": "bim.element.wall",
            "inputJson": serde_json::to_string(&input).unwrap(),
            "nodeHash": 1,
        });
        let out_bytes = extension_invoke("evaluate", req.to_string().as_bytes()).unwrap();
        let out: Dictionary = serde_json::from_slice(&out_bytes).unwrap();
        assert_eq!(channel_payload(&out, "wall").schema(), Some("wall"));
    }
}
// #endregion 🔖️Tests`;

const idx = src.lastIndexOf(insertBefore);
if (idx < 0) {
  console.error("tests end marker not found");
  process.exit(1);
}
src = src.slice(0, idx) + bundleTest + src.slice(idx + insertBefore.length);

fs.writeFileSync(path, src);
console.log("updated", path);
console.log({
  ExtensionBundle: src.includes("ExtensionBundle::new"),
  extension_exports: src.includes("extension_exports!"),
  WasmExt: src.includes("WasmExt"),
  standalone: src.includes("standalone-wasm"),
  PluginGuest: src.includes("PluginGuest"),
});
