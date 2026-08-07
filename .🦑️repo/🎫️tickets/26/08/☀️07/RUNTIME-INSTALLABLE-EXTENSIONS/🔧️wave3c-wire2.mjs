/**
 * Wave 3.c — finish wiring: manifest_json, install_builtin, tests, workspace, cleanup.
 */
import fs from "fs";
import path from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(REPO, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");
const paths = JSON.parse(fs.readFileSync(path.join(TICKET, "wave3c-paths.json"), "utf8"));

function write(file, text, label) {
  fs.writeFileSync(file, text);
  console.log("OK write", label || file);
}

// --- Extension: pub module_registry + extension_manifest_json ---
{
  const extFile = path.join(paths.brepExtRoot, "🦀️component.rs");
  let ext = fs.readFileSync(extFile, "utf8");

  // Make module_registry public if not
  ext = ext.replace(/\nfn module_registry\(\) -> Registry \{/, "\npub fn module_registry() -> Registry {");

  if (!ext.includes("pub fn extension_manifest_json")) {
    const insert = `
/// 🛂️ Manifest JSON for host contribution install (tests + packaging metadata).
pub fn extension_manifest_json() -> String {
    build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
}

`;
    // insert before module_registry
    if (ext.includes("pub fn module_registry()")) {
      ext = ext.replace("pub fn module_registry()", insert + "pub fn module_registry()");
    } else {
      ext = ext.replace("fn module_registry()", insert + "pub fn module_registry()");
    }
    console.log("OK: added extension_manifest_json");
  }

  // Extension guest should use extension_manifest_json()
  ext = ext.replace(
    /let manifest_json = build_manifest_json\("brep", "Brep", "0\.3\.0", &module_registry\(\), vec!\["onStartup"\.into\(\)\], vec!\[\], vec!\[\], vec!\[\]\);/g,
    "let manifest_json = super::extension_manifest_json();",
  );
  // In tests bundle — use extension_manifest_json from super
  ext = ext.replace(
    /let manifest_json = build_manifest_json\("brep", "Brep", "0\.3\.0", &module_registry\(\), vec!\["onStartup"\.into\(\)\], vec!\[\], vec!\[\], vec!\[\]\);/g,
    "let manifest_json = extension_manifest_json();",
  );

  write(extFile, ext, "extension component");
}

// --- install_builtin: remove brep register (empty builtins — all packaged) ---
{
  const core = fs.readFileSync(paths.coreFile, "utf8");
  const next = core.replace(
    `pub fn install_builtin_flow_extensions(registry: &mut neural::Registry) {
    flow_extension_brep::register(registry);
}`,
    `pub fn install_builtin_flow_extensions(_registry: &mut neural::Registry) {
    // Brep/draw and light operator packs are runtime-installable extensions (not compile-time builtins).
}`,
  );
  if (next === core) console.log("FAIL: install_builtin replace");
  else write(paths.coreFile, next, "install_builtin empty");
}

// --- test helper: use brep extension crate ---
{
  let core = fs.readFileSync(paths.coreFile, "utf8");
  const oldBlock = `            for (plugin_id, manifest) in [
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
            semio_s_plugin_flow_extension_list::register(&mut registry);`;

  const newBlock = `            for (plugin_id, manifest) in [
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
            semio_s_plugin_flow_extension_brep::register(&mut registry);
            semio_s_plugin_flow_extension_core::register(&mut registry);
            semio_s_plugin_flow_extension_math::register(&mut registry);
            semio_s_plugin_flow_extension_text::register(&mut registry);
            semio_s_plugin_flow_extension_logic::register(&mut registry);
            semio_s_plugin_flow_extension_dictionary::register(&mut registry);
            semio_s_plugin_flow_extension_list::register(&mut registry);`;

  if (!core.includes(oldBlock)) {
    console.log("FAIL: test helper block not found — dumping nearby");
    const idx = core.indexOf("install_first_party_light_flow_extensions_for_tests");
    fs.writeFileSync(path.join(TICKET, "core-test-helper-snip.rs"), core.slice(idx, idx + 1500));
  } else {
    core = core.replace(oldBlock, newBlock);
    write(paths.coreFile, core, "test helper brep extension");
  }
}

// --- flow Cargo.toml: keep semio-s-3d (geometry session needs it), add brep dev-dep ---
{
  let cargo = fs.readFileSync(paths.flowCargo, "utf8");
  const brepDev =
    'semio-s-plugin-flow-extension-brep = { path = "../../../../../../../✏️s/🔌️plugins/🌊️flow/️️extensions/📐️brep/📦️packages/🦀️rust" }\n';
  // fix emoji path - use actual from paths
  const relDev = path.relative(path.dirname(paths.flowCargo), paths.brepRust).split(path.sep).join("/");
  const brepDevLine = `semio-s-plugin-flow-extension-brep = { path = "${relDev}" }\n`;
  if (!cargo.includes("semio-s-plugin-flow-extension-brep")) {
    cargo = cargo.replace(
      "[dev-dependencies]\n",
      `[dev-dependencies]\n${brepDevLine}`,
    );
    write(paths.flowCargo, cargo, "flow cargo brep dev-dep");
  } else console.log("SKIP flow cargo already has brep");
}

// --- root Cargo.toml workspace member ---
{
  const rootCargoPath = path.join(REPO, "Cargo.toml");
  let root = fs.readFileSync(rootCargoPath, "utf8");
  const memberRel = path.relative(REPO, paths.brepRust).split(path.sep).join("/");
  const memberLine = `    "${memberRel}",`;
  if (root.includes(memberRel)) {
    console.log("SKIP root member exists");
  } else {
    // insert after bim member if present
    const bimNeedle = 'extensions/🏗️bim/📦️packages/🦀️rust"';
    const bimIdx = root.indexOf(bimNeedle);
    if (bimIdx >= 0) {
      const lineEnd = root.indexOf("\n", bimIdx);
      root = root.slice(0, lineEnd + 1) + memberLine + "\n" + root.slice(lineEnd + 1);
      write(rootCargoPath, root, "root Cargo member after bim");
    } else {
      // insert in members array somehow near flow
      const flowPlugin = 'plugins/🌊️flow/📦️packages/🦀️rust"';
      const idx = root.indexOf(flowPlugin);
      if (idx >= 0) {
        const lineEnd = root.indexOf("\n", idx);
        root = root.slice(0, lineEnd + 1) + memberLine + "\n" + root.slice(lineEnd + 1);
        write(rootCargoPath, root, "root Cargo member after flow plugin");
      } else {
        console.log("FAIL: could not find insertion point for workspace member");
        fs.writeFileSync(path.join(TICKET, "root-cargo-snip.txt"), root.split("\n").filter((l) => /flow|bim|brep|extension/.test(l)).join("\n"));
      }
    }
  }
}

// --- package.json: remove stale brep workspace ---
{
  const pkgPath = path.join(REPO, "package.json");
  let pkg = fs.readFileSync(pkgPath, "utf8");
  const lines = pkg.split("\n");
  const next = lines.filter((l) => !(l.includes("extensions") && l.includes("brep") && !l.includes("plugins")));
  if (next.length !== lines.length) {
    write(pkgPath, next.join("\n"), "package.json remove framework brep workspace");
  } else console.log("SKIP package.json brep workspace (already gone or different)");
}

// --- Delete old framework brep extension files ---
{
  const oldDir = paths.brepDir;
  for (const f of fs.readdirSync(oldDir)) {
    const fp = path.join(oldDir, f);
    // keep nothing — remove package.json and old component (we have backup in ticket)
    if (f.includes("component") || f === "package.json") {
      fs.unlinkSync(fp);
      console.log("deleted", fp);
    }
  }
  // remove dir if empty
  try {
    fs.rmdirSync(oldDir);
    console.log("removed dir", oldDir);
  } catch (e) {
    console.log("dir not empty or missing", e.message);
  }
}

console.log("phase B done");
