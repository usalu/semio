import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const gluePath = path.join(flowFw, "📦️packages", "🦀️rust", "📦️glue.rs");
const corePath = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs");
const fwExt = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("extensions")));
const hasFwBrep = fs.readdirSync(fwExt).some((n) => n.includes("brep"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const hasPluginBrep = fs.readdirSync(pluginExt).some((n) => n.includes("brep"));
console.log({ hasFwBrep, hasPluginBrep, fwExt: fs.readdirSync(fwExt) });

const coreName = fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions"));
const wasmName = fs.readdirSync(fwExt).find((n) => n.includes("wasm"));
const fwExtName = path.basename(fwExt);
const hasBrepGeometry = fs.existsSync(path.join(flowFw, coreName, fs.readdirSync(path.join(flowFw, coreName)).find((n) => n.includes("brep-geometry") || false) || "", "🦀️component.rs"))
  || fs.readdirSync(path.join(flowFw, coreName)).some((n) => n.includes("brep-geometry"));
const brepGeomDir = fs.readdirSync(path.join(flowFw, coreName)).find((n) => n.includes("brep-geometry"));

// Rewrite glue for current reality: no fw brep operators; optional brep_geometry; wasm SDK
let glue = `//! 🌊️ OS flow family glue — wires core, brep geometry kernel surface, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow_core;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../${coreName}/🦀️component.rs"]
pub mod core;
pub use core::*;
`;

if (brepGeomDir) {
  glue += `
#[path = "../../${coreName}/${brepGeomDir}/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};
`;
}

glue += `
#[path = "."]
pub mod extensions {
  #[path = "../../${fwExtName}/${wasmName}/🦀️component.rs"]
  pub mod wasm;
}

pub use extensions::wasm::*;
`;
fs.writeFileSync(gluePath, glue);
console.log("rewrote glue without missing brep path-mod");

// Fix install_builtin (keep empty) and test helper (drop flow_extension_brep; optionally register packaged brep)
let core = fs.readFileSync(corePath, "utf8");

// Ensure install_builtin is empty
core = core.replace(
  /pub fn install_builtin_flow_extensions\([^\)]*\) \{[\s\S]*?\n\}/,
  `pub fn install_builtin_flow_extensions(_registry: &mut neural::Registry) {
    // Light/draw/brep operator packs are runtime-installable packaged extensions.
}`,
);

// Fix helper: remove flow_extension_brep::register; add packaged brep if available as dev-dep
const flowCargoPath = path.join(flowFw, "📦️packages", "🦀️rust", "Cargo.toml");
let flowCargo = fs.readFileSync(flowCargoPath, "utf8");
const brepDevDep = `semio-s-plugin-flow-extension-brep = { path = "../../../../../../../✏️s/🔌️plugins/🌊️flow/${path.basename(pluginExt)}/${fs.readdirSync(pluginExt).find((n) => n.includes("brep"))}/📦️packages/🦀️rust", default-features = false }`;
if (hasPluginBrep && !flowCargo.includes("semio-s-plugin-flow-extension-brep")) {
  if (!flowCargo.includes("[dev-dependencies]")) flowCargo += "\n[dev-dependencies]\n";
  flowCargo = flowCargo.replace("[dev-dependencies]", `[dev-dependencies]\n${brepDevDep}`);
  fs.writeFileSync(flowCargoPath, flowCargo);
  console.log("added brep dev-dep for fixture tests");
}

const helper = `
    /// 🧪️ Installs first-party light (+brep) flow extension manifests and real in-process ops for fixture tests.
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
                ("flow-extension-brep", semio_s_plugin_flow_extension_brep::extension_manifest_json()),
            ] {
                install_flow_extension_manifest(plugin_id, &manifest);
            }
            let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
            let mut registry = neural::Registry::new();
            semio_s_plugin_flow_extension_core::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);
            semio_s_plugin_flow_extension_brep::register(&mut registry);
            state.registry = std::sync::Arc::new(registry);
            state.generation += 1;
        });
    }
`;

if (core.includes("fn install_first_party_light_flow_extensions_for_tests")) {
  core = core.replace(/fn install_first_party_light_flow_extensions_for_tests\(\) \{[\s\S]*?\n    \}\n/, helper.trimStart() + "\n");
} else {
  core = core.replace("fn fixture_kind_infos_json() -> String {", helper + "\n    fn fixture_kind_infos_json() -> String {");
}

// Remove any remaining flow_extension_brep:: / flow_extension_draw:: that would not resolve
const leftover = [];
core.split(/\n/).forEach((l, i) => {
  if (/flow_extension_brep::|flow_extension_draw::|flow_extension_math::|flow_extension_core::|flow_extension_text::|flow_extension_logic::|flow_extension_dictionary::|flow_extension_list::/.test(l)) {
    leftover.push(`${i + 1}:${l.trim()}`);
  }
});
console.log("leftover crate refs", leftover);

fs.writeFileSync(corePath, core);
console.log("updated core helper + empty install_builtin");

// Verify brep package has extension_manifest_json
const brepComp = path.join(pluginExt, fs.readdirSync(pluginExt).find((n) => n.includes("brep")), "🦀️component.rs");
const brepText = fs.readFileSync(brepComp, "utf8");
console.log("brep has extension_manifest_json", brepText.includes("extension_manifest_json"));
console.log("brep has register", /pub fn register\(/.test(brepText));
