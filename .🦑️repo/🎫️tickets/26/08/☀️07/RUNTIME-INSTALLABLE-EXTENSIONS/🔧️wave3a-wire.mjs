import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const glue = path.join(flowFw, "📦️packages", "🦀️rust", "📦️glue.rs");
const core = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs");
const flowPluginCargo = path.join(root, sDir, "🔌️plugins", "🌊️flow", "📦️packages", "🦀️rust", "Cargo.toml");
const flowApp = path.join(root, sDir, "🔌️plugins", "🌊️flow", "🎛️apps", "🌊️flow", "🦀️component.rs");
const cargoToml = path.join(root, "Cargo.toml");
const flowPkgCargo = path.join(flowFw, "📦️packages", "🦀️rust", "Cargo.toml");

const SIX = ["core", "math", "text", "logic", "dictionary", "list"];
const extPluginRoot = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));

// 1) glue.rs — remove six path mods
let glueText = `//! 🌊️ OS flow family glue — wires core and remaining built-in extensions (brep/draw) plus wasm SDK.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_draw;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../${fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions"))}/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "."]
pub mod extensions {
  #[path = "../../../${fs.readdirSync(path.join(flowFw, fs.readdirSync(flowFw).find(n=>n.includes("extensions")))).find(n=>n.includes("brep"))}/🦀️component.rs"]
  pub mod brep;

  #[path = "../../../${fs.readdirSync(path.join(flowFw, fs.readdirSync(flowFw).find(n=>n.includes("extensions")))).find(n=>n.includes("wasm"))}/🦀️component.rs"]
  pub mod wasm;

  #[path = "../../../${fs.readdirSync(path.join(flowFw, fs.readdirSync(flowFw).find(n=>n.includes("extensions")))).find(n=>n.includes("draw"))}/🦀️component.rs"]
  pub mod draw;
}

pub use extensions::brep::*;
pub use extensions::draw::*;
pub use extensions::wasm::*;
`;
fs.writeFileSync(glue, glueText);
console.log("updated glue");

// 2) install_builtin — only brep + draw
let coreText = fs.readFileSync(core, "utf8");
coreText = coreText.replace(
  /pub fn install_builtin_flow_extensions\(registry: &mut neural::Registry\) \{[\s\S]*?\n\}/,
  `pub fn install_builtin_flow_extensions(registry: &mut neural::Registry) {
    flow_extension_brep::register(registry);
    flow_extension_draw::register(registry);
}`,
);

// 3) fixture_kind_infos_json — use brep only + install light manifests via dev-deps in tests
// Replace fixture_kind_infos_json body
coreText = coreText.replace(
  /fn fixture_kind_infos_json\(\) -> String \{[\s\S]*?\n    \}/,
  `fn fixture_kind_infos_json() -> String {
        install_first_party_light_flow_extensions_for_tests();
        let mut registry = Registry::new();
        flow_extension_brep::register(&mut registry);
        serde_json::to_string(&flow_extension_registry().operator_catalogue()).unwrap_or_else(|_| "[]".into())
    }`,
);

// Add test helper near other test helpers — after RECTANGLE lock or before fixture_kind_infos
if (!coreText.includes("install_first_party_light_flow_extensions_for_tests")) {
  const helper = `
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
            // Real in-process ops for fixture evaluation (manifest install only yields PendingExtension stubs).
            let mut composed = neural::Registry::new();
            flow_extension_brep::register(&mut composed);
            flow_extension_draw::register(&mut composed);
            semio_s_plugin_flow_extension_core::register(&mut composed);
            semio_s_plugin_flow_extension_math::register(&mut composed);
            semio_s_plugin_flow_extension_text::register(&mut composed);
            semio_s_plugin_flow_extension_logic::register(&mut composed);
            semio_s_plugin_flow_extension_dictionary::register(&mut composed);
            semio_s_plugin_flow_extension_list::register(&mut composed);
            // Re-apply contributed manifests for any extra schemas already recorded.
            let state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
            for entry in state.contributed.values() {
                let _ = entry;
            }
            drop(state);
            let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
            // Swap registry to real ops for tests while keeping contributed bookkeeping.
            // Builtins already inside composed; re-register contributed as stubs on top would override — so register real ops for light ids by rebuilding:
            let mut registry = neural::Registry::new();
            flow_extension_brep::register(&mut registry);
            flow_extension_draw::register(&mut registry);
            semio_s_plugin_flow_extension_core::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);
            for entry in state.contributed.values() {
                register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json);
            }
            state.registry = std::sync::Arc::new(registry);
            state.generation += 1;
        });
    }
`;
  // This helper is too complex / wrong order. Simplify approach below — rewrite helper simpler.
}

fs.writeFileSync(core, coreText);
console.log("patched core install_builtin (helper pending simplify)");

// Simpler approach for tests: replace fixture helper properly in a second pass
let core2 = fs.readFileSync(core, "utf8");
// Remove botched helper if partially inserted
core2 = core2.replace(/\n    fn install_first_party_light_flow_extensions_for_tests\(\) \{[\s\S]*?\n    \}\n/g, "\n");

const simpleHelper = `
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

if (!core2.includes("install_first_party_light_flow_extensions_for_tests")) {
  core2 = core2.replace(
    "fn fixture_kind_infos_json() -> String {",
    simpleHelper + "\n    fn fixture_kind_infos_json() -> String {",
  );
}

// Fix fixture_kind_infos_json to call helper and use registry catalogue
core2 = core2.replace(
  /fn fixture_kind_infos_json\(\) -> String \{[\s\S]*?\n    \}/,
  `fn fixture_kind_infos_json() -> String {
        install_first_party_light_flow_extensions_for_tests();
        serde_json::to_string(&flow_extension_registry().operator_catalogue()).unwrap_or_else(|_| "[]".into())
    }`,
);

// Tests that need math in catalogue from global registry
const testsNeedingSeed = [
  "fn catalogue_has_module_sections",
  "fn flow_backed_node_graph_extras_include_fixture_and_flow_engine",
  "fn flow_fixture_with_synapses_builds_dag_edges_and_ports",
];
// For catalogue_has_module_sections - uses host_with_test_bridge which has math in test catalogue - OK
// For flow_backed - uses flow_extension_registry - needs seed at start of test
core2 = core2.replace(
  "fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {\n        let host = host_with_test_bridge();",
  "fn flow_backed_node_graph_extras_include_fixture_and_flow_engine() {\n        install_first_party_light_flow_extensions_for_tests();\n        let host = host_with_test_bridge();",
);

// Tests using flow_neuron_kind_infos_json with math.add
core2 = core2.replace(
  "fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {\n        let mut host = host_with_test_bridge();\n        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());",
  "fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {\n        install_first_party_light_flow_extensions_for_tests();\n        let mut host = host_with_test_bridge();\n        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());",
);

// rectangle/hexagonal already use fixture_kind_infos_json which seeds

fs.writeFileSync(core, core2);
console.log("updated core tests helper");

// 4) framework flow Cargo.toml — add dev-dependencies on six crates
let flowCargo = fs.readFileSync(flowPkgCargo, "utf8");
if (!flowCargo.includes("[dev-dependencies]")) {
  const deps = SIX.map((id) => {
    const dir = fs.readdirSync(extPluginRoot).find((n) => n.endsWith(id));
    return `semio-s-plugin-flow-extension-${id} = { path = "../../../../../../${sDir}/🔌️plugins/🌊️flow/${path.basename(extPluginRoot)}/${dir}/📦️packages/🦀️rust" }`;
  }).join("\n");
  flowCargo += `\n[dev-dependencies]\n${deps}\n`;
  fs.writeFileSync(flowPkgCargo, flowCargo);
  console.log("added flow framework dev-dependencies");
} else {
  console.log("flow cargo already has dev-dependencies section — check manually");
}

// 5) root Cargo.toml workspace members
let rootCargo = fs.readFileSync(cargoToml, "utf8");
const bimMember = rootCargo.split("\n").find((l) => l.includes("flow-extension-bim") || l.includes("flow/️️extensions/🏗️bim") || l.includes("extensions/🏗️bim"));
const insertAfter = rootCargo.includes('flow/️️extensions/🏗️bim') 
  ? rootCargo.match(/.*"✏️s\/🔌️plugins\/🌊️flow\/[^"]+bim[^"]+".*/)[0]
  : rootCargo.match(/.*"✏️s\/🔌️plugins\/🌊️flow\/[^"]*bim[^"]+".*/)[0];
console.log("bim member line:", insertAfter);
const newMembers = SIX.map((id) => {
  const dir = fs.readdirSync(extPluginRoot).find((n) => n.endsWith(id));
  return `    "✏️s/🔌️plugins/🌊️flow/${path.basename(extPluginRoot)}/${dir}/📦️packages/🦀️rust",`;
}).join("\n");
if (!rootCargo.includes("flow-extension-math") && !rootCargo.includes(`extensions/🧮️math`)) {
  rootCargo = rootCargo.replace(insertAfter, insertAfter + "\n" + newMembers);
  fs.writeFileSync(cargoToml, rootCargo);
  console.log("added workspace members");
}

// 6) flow plugin Cargo.toml — consumes + optional deps for testkit
let pluginCargo = fs.readFileSync(flowPluginCargo, "utf8");
if (!pluginCargo.includes('consumes')) {
  pluginCargo = pluginCargo.replace(
    /\[package\.metadata\.semio\]\nrole = "plugin"/,
    `[package.metadata.semio]\nrole = "plugin"\nconsumes = ["flow.extension"]`,
  );
}
// Add deps on six for testkit seeding
if (!pluginCargo.includes("semio-s-plugin-flow-extension-math")) {
  const deps = SIX.map((id) => {
    const dir = fs.readdirSync(extPluginRoot).find((n) => n.endsWith(id));
    return `semio-s-plugin-flow-extension-${id} = { path = "../../${path.basename(extPluginRoot)}/${dir}/📦️packages/🦀️rust" }`;
  }).join("\n");
  // Prefer [dev-dependencies]
  if (!pluginCargo.includes("[dev-dependencies]")) {
    pluginCargo += `\n[dev-dependencies]\n${deps}\n`;
  } else {
    pluginCargo = pluginCargo.replace("[dev-dependencies]", `[dev-dependencies]\n${deps}`);
  }
}
fs.writeFileSync(flowPluginCargo, pluginCargo);
console.log("updated flow plugin cargo");

// 7) flow_app testkit seed
let appText = fs.readFileSync(flowApp, "utf8");
if (!appText.includes("install_first_party_light_flow_extensions_for_tests")) {
  const seedFn = `
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
                flow_core::install_flow_extension_manifest(plugin_id, &manifest);
            }
        });
    }
`;
  appText = appText.replace(
    "pub fn flow_app() -> FlowApp {\n        new_app::<FlowPlayApp>()\n    }",
    `pub fn flow_app() -> FlowApp {\n        install_first_party_light_flow_extensions_for_tests();\n        new_app::<FlowPlayApp>()\n    }`,
  );
  appText = appText.replace(
    "pub fn flow_app_with_registry() -> FlowApp {\n        new_app_with_registry::<FlowPlayApp>(create_flow_app)\n    }",
    `pub fn flow_app_with_registry() -> FlowApp {\n        install_first_party_light_flow_extensions_for_tests();\n        new_app_with_registry::<FlowPlayApp>(create_flow_app)\n    }`,
  );
  appText = appText.replace(
    "pub type FlowApp = VcsDocumentApp<FlowPlayApp>;",
    `pub type FlowApp = VcsDocumentApp<FlowPlayApp>;\n${seedFn}`,
  );
  fs.writeFileSync(flowApp, appText);
  console.log("updated flow testkit");
}

console.log("wire complete");
