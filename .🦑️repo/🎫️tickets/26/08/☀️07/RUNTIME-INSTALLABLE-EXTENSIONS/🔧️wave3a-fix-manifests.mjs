import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const extRoot = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));

function extractBalancedCall(t, startIdx) {
  const open = t.indexOf("(", startIdx);
  let depth = 0;
  for (let i = open; i < t.length; i++) {
    const ch = t[i];
    if (ch === "(") depth++;
    else if (ch === ")") {
      depth--;
      if (depth === 0) return t.slice(startIdx, i + 1);
    }
  }
  return null;
}

const SPECS = {
  core: { imports: ["build_manifest_json"], fallback: `build_manifest_json("core", "Core", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])` },
  math: { imports: ["build_manifest_json", "FlowExtensionCommand", "FlowExtensionSetting"], fallback: null },
  text: { imports: ["build_manifest_json", "FlowExtensionCommand"], fallback: `build_manifest_json("text", "Text", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![])` },
  logic: { imports: ["build_manifest_json", "FlowExtensionCommand"], fallback: `build_manifest_json("logic", "Logic", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "logic.showHelp".into(), title: "Logic: Show Help".into() }], vec![])` },
  dictionary: { imports: ["build_manifest_json", "FlowExtensionCommand"], fallback: `build_manifest_json("dictionary", "Dictionary", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }], vec![])` },
  list: { imports: ["build_manifest_json", "FlowExtensionCommand"], fallback: `build_manifest_json("list", "List", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "list.showHelp".into(), title: "List: Show Help".into() }], vec![])` },
};

for (const [id, spec] of Object.entries(SPECS)) {
  const dir = fs.readdirSync(extRoot).find((n) => n.endsWith(id));
  const comp = path.join(extRoot, dir, "🦀️component.rs");
  let t = fs.readFileSync(comp, "utf8");

  // Strip broken Manifest region
  t = t.replace(/\/\/ #region 🔖️Manifest[\s\S]*?\/\/ #endregion 🔖️Manifest\n*/g, "");

  // Collect complete build_manifest_json calls from remaining file (tests)
  const idxs = [];
  let from = 0;
  while (true) {
    const i = t.indexOf("build_manifest_json(", from);
    if (i < 0) break;
    idxs.push(i);
    from = i + 1;
  }
  let call = null;
  for (const i of idxs) {
    const c = extractBalancedCall(t, i);
    if (!c) continue;
    if ((c.match(/,/g) || []).length >= 6) {
      call = c.replace(/&reg\b/g, "&module_registry()");
    }
  }
  if (!call) call = spec.fallback;
  if (!call) {
    console.error("no call for", id, "idxs", idxs.length);
    // dump around first
    if (idxs[0] != null) console.error(t.slice(idxs[0], idxs[0] + 300));
    process.exit(1);
  }

  // For math, prefer the multi-line one with settings
  if (id === "math") {
    for (const i of idxs) {
      const c = extractBalancedCall(t, i);
      if (c && c.includes("FlowExtensionSetting")) {
        call = c.replace(/&reg\b/g, "&module_registry()");
      }
    }
  }

  const region = `// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{${spec.imports.join(", ")}};
    ${call}
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> neural_engine::Registry {
    let mut registry = neural_engine::Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

`;

  if (t.includes("// #region 🔖️Tests")) {
    t = t.replace("// #region 🔖️Tests", region + "// #region 🔖️Tests");
  } else {
    t = t.replace("// #region 🔖️ExtensionGuest", region + "// #region 🔖️ExtensionGuest");
  }

  fs.writeFileSync(comp, t);
  console.log("OK", id, call.replace(/\s+/g, " ").slice(0, 160));
}
