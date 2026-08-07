import { readdirSync, readFileSync, writeFileSync, existsSync } from "fs";
import { join, relative } from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = `${REPO}/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS`;

const mods = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules");
const flowMod = join(mods, readdirSync(mods).find((n) => n.includes("flow")));
const flowRust = join(flowMod, "📦️packages/🦀️rust");
const fext = join(flowMod, readdirSync(flowMod).find((n) => n.includes("extensions")));
const flowPlugin = join(REPO, "✏️s/🔌️plugins/🌊️flow");
const pextName = readdirSync(flowPlugin).find((n) => n.includes("extensions"));
const pext = join(flowPlugin, pextName);

const brepName = readdirSync(fext).find((n) => n.includes("brep"));
const wasmName = readdirSync(fext).find((n) => n.includes("wasm"));
const drawName = readdirSync(pext).find((n) => n.includes("draw"));
const coreName = readdirSync(flowMod).find((n) => n.includes("core") && !n.includes("extension"));

const brepRel = relative(flowRust, join(fext, brepName, "🦀️component.rs")).split("\\").join("/");
const wasmRel = relative(flowRust, join(fext, wasmName, "🦀️component.rs")).split("\\").join("/");
const drawRel = relative(flowRust, join(pext, drawName, "🦀️component.rs")).split("\\").join("/");
const coreRel = relative(flowRust, join(flowMod, coreName, "🦀️component.rs")).split("\\").join("/");

console.log({ brepRel, wasmRel, drawRel, coreRel });

const glue = `//! 🌊️ OS flow family glue — wires core, brep/draw builtins, and wasm SDK helpers.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_draw;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "${coreRel}"]
pub mod core;
pub use core::*;

#[path = "."]
pub mod extensions {
  #[path = "${brepRel}"]
  pub mod brep;

  #[path = "${wasmRel}"]
  pub mod wasm;

  #[path = "${drawRel}"]
  pub mod draw;
}

pub use extensions::brep::*;
pub use extensions::draw::*;
pub use extensions::wasm::*;
`;
writeFileSync(join(flowRust, "📦️glue.rs"), glue);
console.log("wrote glue");

// Patch core tests: add helper after OnceLock static
const corePath = join(flowMod, coreName, "🦀️component.rs");
let core = readFileSync(corePath, "utf8");

const helper = `
    /// 🧪 Installs first-party light flow extensions into the process registry for host unit tests.
    fn install_first_party_light_flow_extensions_for_tests() {
        install_flow_extension(FlowExtensionSpec {
            id: "core".into(),
            name: "Core".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_core::register,
        });
        install_flow_extension(FlowExtensionSpec {
            id: "math".into(),
            name: "Math".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_math::register,
        });
        install_flow_extension(FlowExtensionSpec {
            id: "text".into(),
            name: "Text".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_text::register,
        });
        install_flow_extension(FlowExtensionSpec {
            id: "logic".into(),
            name: "Logic".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_logic::register,
        });
        install_flow_extension(FlowExtensionSpec {
            id: "dictionary".into(),
            name: "Dictionary".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_dictionary::register,
        });
        install_flow_extension(FlowExtensionSpec {
            id: "list".into(),
            name: "List".into(),
            version: "0.1.0".into(),
            install: semio_s_plugin_flow_extension_list::register,
        });
    }
`;

if (!core.includes("fn install_first_party_light_flow_extensions_for_tests()")) {
  const anchor = "static RECTANGLE_EXTRUDE_FIXTURE_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();";
  if (!core.includes(anchor)) throw new Error("missing OnceLock anchor");
  core = core.replace(anchor, anchor + "\n" + helper);
  writeFileSync(corePath, core);
  console.log("inserted test helper");
} else {
  console.log("helper already present");
}

// Verify install_builtin only has brep+draw
const builtin = core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/);
console.log("builtin:\n", builtin?.[0]);

// Root Cargo.toml members
const rootPath = join(REPO, "Cargo.toml");
let root = readFileSync(rootPath, "utf8");
const lightIds = [
  ["core", readdirSync(pext).find((n) => n.includes("core"))],
  ["math", readdirSync(pext).find((n) => n.includes("math"))],
  ["text", readdirSync(pext).find((n) => n.includes("text"))],
  ["logic", readdirSync(pext).find((n) => n.includes("logic"))],
  ["dictionary", readdirSync(pext).find((n) => n.includes("dictionary"))],
  ["list", readdirSync(pext).find((n) => n.includes("list"))],
];

const memberLines = lightIds.map(
  ([, dir]) => `    "✏️s/🔌️plugins/🌊️flow/${pextName}/${dir}/📦️packages/🦀️rust",`,
);

// Insert after draw member if present, else after bim
let inserted = 0;
for (const line of memberLines) {
  if (root.includes(line.trim())) {
    console.log("member exists", line.trim());
    continue;
  }
  const drawMember = root.split("\n").find((l) => l.includes("🌊️flow") && l.includes("extensions") && l.includes("draw") && l.includes("packages"));
  const bimMember = root.split("\n").find((l) => l.includes("🌊️flow") && l.includes("extensions") && l.includes("bim") && l.includes("packages"));
  const anchorLine = drawMember || bimMember;
  if (!anchorLine) throw new Error("no bim/draw member anchor");
  root = root.replace(anchorLine, anchorLine + "\n" + line);
  inserted++;
  console.log("added member", line.trim());
}

// workspace aliases near draw alias if present
const aliasBlock = lightIds
  .map(
    ([id, dir]) =>
      `semio-s-plugin-flow-extension-${id} = { path = "✏️s/🔌️plugins/🌊️flow/${pextName}/${dir}/📦️packages/🦀️rust" }`,
  )
  .join("\n");

if (!root.includes("semio-s-plugin-flow-extension-math")) {
  const drawAlias = root.split("\n").find((l) => l.startsWith("semio-s-plugin-flow-extension-draw"));
  if (drawAlias) {
    root = root.replace(drawAlias, aliasBlock + "\n" + drawAlias);
    console.log("added workspace aliases before draw");
  } else {
    // append near other plugin aliases
    const bimPathAlias = root.split("\n").find((l) => l.includes("semio-s-plugin-flow") && l.includes("path") && !l.includes("extension"));
    // insert after members section's related deps — find flow-extension-bim path dep if any
    const insertAt = root.indexOf("semio-s-plugin-flow-extension-draw");
    if (insertAt >= 0) {
      root = root.slice(0, insertAt) + aliasBlock + "\n" + root.slice(insertAt);
    } else {
      const marker = '[workspace.dependencies]';
      const mi = root.indexOf(marker);
      if (mi < 0) throw new Error("no workspace.dependencies");
      const after = root.indexOf("\n", mi) + 1;
      root = root.slice(0, after) + aliasBlock + "\n" + root.slice(after);
    }
    console.log("added workspace aliases");
  }
} else {
  console.log("aliases already present");
}

writeFileSync(rootPath, root);
console.log("members inserted", inserted);

// Sync twin ticket folder summary later; copy glue snap
writeFileSync(join(TICKET, "snap-glue-fixed.rs"), glue);
writeFileSync(
  join(TICKET, "wave3a-paths.json"),
  JSON.stringify({ brepRel, wasmRel, drawRel, coreRel, members: memberLines, pextName, lightIds }, null, 2),
);
console.log("DONE");
