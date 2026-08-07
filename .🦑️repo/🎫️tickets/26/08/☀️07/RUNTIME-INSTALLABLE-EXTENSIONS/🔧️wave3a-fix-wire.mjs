import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const fwExtName = fs.readdirSync(flowFw).find((n) => n.includes("extensions"));
const fwExt = path.join(flowFw, fwExtName);
const coreName = fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions"));
const brep = fs.readdirSync(fwExt).find((n) => n.includes("brep"));
const wasm = fs.readdirSync(fwExt).find((n) => n.includes("wasm"));
const draw = fs.readdirSync(fwExt).find((n) => n.includes("draw"));
if (!brep || !wasm || !draw) throw new Error(JSON.stringify({ brep, wasm, draw, listing: fs.readdirSync(fwExt) }));

const glue = path.join(flowFw, "📦️packages", "🦀️rust", "📦️glue.rs");
fs.writeFileSync(
  glue,
  `//! 🌊️ OS flow family glue — wires core and remaining built-in extensions (brep/draw) plus wasm SDK.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_draw;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../${coreName}/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "."]
pub mod extensions {
  #[path = "../../../${fwExtName}/${brep}/🦀️component.rs"]
  pub mod brep;

  #[path = "../../../${fwExtName}/${wasm}/🦀️component.rs"]
  pub mod wasm;

  #[path = "../../../${fwExtName}/${draw}/🦀️component.rs"]
  pub mod draw;
}

pub use extensions::brep::*;
pub use extensions::draw::*;
pub use extensions::wasm::*;
`,
);
console.log("fixed glue", { coreName, fwExtName, brep, wasm, draw });

// Workspace members
const pluginExtName = fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow", pluginExtName);
const SIX = ["core", "math", "text", "logic", "dictionary", "list"];
let cargo = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
const bimLine = cargo.split("\n").find((l) => l.includes("bim") && l.includes("flow") && l.includes("extensions"));
if (!bimLine) throw new Error("no bim member");
const members = SIX.map((id) => {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  return `    "✏️s/🔌️plugins/🌊️flow/${pluginExtName}/${dir}/📦️packages/🦀️rust",`;
});
let added = 0;
for (const m of members) {
  if (!cargo.includes(m.trim().replace(/,$/, ""))) {
    cargo = cargo.replace(bimLine, bimLine + "\n" + m);
    added++;
  }
}
fs.writeFileSync(path.join(root, "Cargo.toml"), cargo);
console.log("workspace members added", added);

// Ensure helper exists in core
const corePath = path.join(flowFw, coreName, "🦀️component.rs");
let core = fs.readFileSync(corePath, "utf8");
if (!core.includes("fn install_first_party_light_flow_extensions_for_tests")) {
  const helper = `
    /// 🧪️ Installs first-party light flow extension manifests and real in-process ops for fixture tests.
    fn install_first_party_light_flow_extensions_for_tests() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            for (plugin_id, manifest) in [
                ("flow-extension-core", semio_s_plugin_flow_extension_core::extension_manifest_json()),
                ("flow-extension-math", semio_s_plugin_flow_extension_math::extension_manifest_json()),
                ("flow-extension-text", semio_s_plugin_flow_extension_text::extension_manifest_json()),
                ("flow-extension-logic", semio_s_plugin_flow_extension_logic::extension_manifest_json()),
                ("flow-extension-dictionary", semio_s_plugin_flow_extension_dictionary::extension_manifest_json()),
                ("flow-extension-list", semio_s_plugin_flow_extension_list::extension_manifest_json()),
            ] {
                install_flow_extension_manifest(plugin_id, &manifest);
            }
            let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
            let mut registry = neural::Registry::new();
            flow_extension_brep::register(&mut registry);
            flow_extension_draw::register(&mut registry);
            semio_s_plugin_flow_extension_core::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);
            state.registry = std::sync::Arc::new(registry);
            state.generation += 1;
        });
    }

`;
  if (!core.includes("fn fixture_kind_infos_json")) throw new Error("no fixture_kind_infos_json");
  core = core.replace("fn fixture_kind_infos_json() -> String {", helper + "    fn fixture_kind_infos_json() -> String {");
  fs.writeFileSync(corePath, core);
  console.log("inserted helper");
} else {
  console.log("helper already present");
}

// Fix list manifest to use showHelp for consistency if it's list.test from a weird test
const listDir = fs.readdirSync(pluginExt).find((n) => n.endsWith("list"));
const listComp = path.join(pluginExt, listDir, "🦀️component.rs");
let list = fs.readFileSync(listComp, "utf8");
list = list.replace(
  /pub fn extension_manifest_json\(\) -> String \{[\s\S]*?\n\}/,
  `pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};
    build_manifest_json("list", "List", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "list.showHelp".into(), title: "List: Show Help".into() }], vec![])
}`,
);
fs.writeFileSync(listComp, list);

// core manifest shouldn't import unused FlowExtensionCommand
const coreExtDir = fs.readdirSync(pluginExt).find((n) => n.endsWith("core"));
const coreExt = path.join(pluginExt, coreExtDir, "🦀️component.rs");
let coreExtText = fs.readFileSync(coreExt, "utf8");
coreExtText = coreExtText.replace(
  "use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};",
  "use flow_extension_sdk::build_manifest_json;",
);
fs.writeFileSync(coreExt, coreExtText);

console.log("done fixes");
