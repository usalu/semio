import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const corePath = path.join(flowFw, fs.readdirSync(flowFw).find((n) => n.includes("core") && !n.includes("extensions")), "🦀️component.rs");
const pluginExtName = fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions"));
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow", pluginExtName);
const SIX = ["core", "math", "text", "logic", "dictionary", "list"];

// Insert helper
let core = fs.readFileSync(corePath, "utf8");
if (core.includes("flow_extension_draw::")) {
  console.log("WARNING: still has flow_extension_draw refs");
  core.split(/\n/).forEach((l,i)=>{ if (l.includes("flow_extension_draw::")) console.log((i+1)+":"+l.trim()); });
}

if (!core.includes("fn install_first_party_light_flow_extensions_for_tests")) {
  const helper = `
    /// 🧪️ Installs first-party light flow extension manifests + real in-process ops for fixture tests.
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
  if (!core.includes("fn fixture_kind_infos_json() -> String")) throw new Error("missing fixture_kind_infos_json");
  core = core.replace("    fn fixture_kind_infos_json() -> String {", helper + "    fn fixture_kind_infos_json() -> String {");
  fs.writeFileSync(corePath, core);
  console.log("inserted helper");
} else {
  console.log("helper ok");
}

// Workspace members after bim line
let cargo = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
const bimLine = cargo.split("\n").find((l) => l.includes("extensions/🏗️bim") || (l.includes("bim") && l.includes("flow") && l.includes("extensions")));
if (!bimLine) throw new Error("no bim");
let added = 0;
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const member = `    "✏️s/🔌️plugins/🌊️flow/${pluginExtName}/${dir}/📦️packages/🦀️rust",`;
  if (!cargo.includes(member) && !cargo.includes(`/${dir}/📦️packages/🦀️rust`)) {
    cargo = cargo.replace(bimLine, bimLine + "\n" + member);
    added++;
  }
}
fs.writeFileSync(path.join(root, "Cargo.toml"), cargo);
console.log("members added", added);

// Verify each extension component compiles-ish: no duplicate module_registry, has guest
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  const t = fs.readFileSync(path.join(pluginExt, dir, "🦀️component.rs"), "utf8");
  const count = (t.match(/fn module_registry/g) || []).length;
  const issues = [];
  if (count !== 1) issues.push("module_registry="+count);
  if (!t.includes("extension_exports!")) issues.push("no exports");
  if (!t.includes("pub fn extension_manifest_json")) issues.push("no manifest fn");
  // check ExtensionGuest uses extension_manifest_json
  if (!t.includes("extension_manifest_json()")) issues.push("guest/manifest unused?");
  console.log(id, issues.length ? issues : "ok");
}
