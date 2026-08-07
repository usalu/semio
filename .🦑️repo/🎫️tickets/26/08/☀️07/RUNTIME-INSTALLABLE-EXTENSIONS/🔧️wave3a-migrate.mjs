import fs from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const flowFw = path.join(root, framework, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const extFw = path.join(flowFw, "🧩️extensions");
const flowPluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow", "️️extensions");
// correct emoji
const flowPluginExtCorrect = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));

const SIX = [
  { id: "core", label: "Core", emojiDir: null, icon: "core", commands: [], settings: false },
  { id: "math", label: "Math", emojiDir: null, icon: "math", commands: true, settings: true },
  { id: "text", label: "Text", emojiDir: null, icon: "text", commands: true, settings: false },
  { id: "logic", label: "Logic", emojiDir: null, icon: "logic", commands: true, settings: false },
  { id: "dictionary", label: "Dictionary", emojiDir: null, icon: "dictionary", commands: true, settings: false },
  { id: "list", label: "List", emojiDir: null, icon: "list", commands: true, settings: false },
];

for (const spec of SIX) {
  const dir = fs.readdirSync(extFw).find((n) => n.endsWith(spec.id) || n.includes(spec.id));
  if (!dir) throw new Error("missing " + spec.id);
  spec.emojiDir = dir;
  spec.src = path.join(extFw, dir, "🦀️component.rs");
}

console.log(SIX.map((s) => ({ id: s.id, dir: s.emojiDir })));

function titleCase(id) {
  return id.charAt(0).toUpperCase() + id.slice(1);
}

function buildGuestRegion(spec, manifestExpr) {
  return `
// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for \`${spec.id}\`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::evaluate_json;
    use semio_framework_core::{Contribution, Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::ExtensionBundle;
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "${spec.id}";
    const EXTENSION_LABEL: &str = "${spec.label}";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn flow_extension_contribution(app_id: &str, manifest_json: String) -> Contribution {
        Contribution::FlowExtension {
            app_id: app_id.into(),
            extension_id: EXTENSION_ID.into(),
            label: EXTENSION_LABEL.into(),
            icon_id: "${spec.icon}".into(),
            manifest_json,
        }
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        ExtensionBundle::new(EXTENSION_ID, EXTENSION_LABEL, "0.1.0")
            .extends("flow")
            .contributes(flow_extension_contribution(FLOW_APP_ID, manifest_json.clone()))
            .contributes(flow_extension_contribution(PROCEDURAL3D_APP_ID, manifest_json))
            .handler("evaluate", |req| {
                let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string())
                })?;
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            })
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
`;
}

function ensureManifestFn(src, spec) {
  let t = src;
  // Make module_registry always available
  t = t.replace(/#\[cfg\(any\(test, target_arch = "wasm32"\)\)\]\s*\n\s*fn module_registry\(\)/g, "fn module_registry()");
  t = t.replace(/#\[cfg\(any\(test, target_arch = "wasm32"\)\)\]\s*\nfn module_registry\(\)/g, "fn module_registry()");
  t = t.replace(/#\[cfg\(any\(test, target_arch = "wasm32"\)\)\]\nfn module_registry\(\)/g, "fn module_registry()");

  // Remove WasmExt region entirely
  t = t.replace(/\n\/\/ #region 🔖️WasmExt[\s\S]*?\/\/ #endregion 🔖️WasmExt\n?/g, "\n");

  if (!t.includes("fn extension_manifest_json")) {
    // Extract build_manifest_json call from tests if present, else synthesize
    let manifestBody;
    const m = t.match(/build_manifest_json\(([\s\S]*?)\)/);
    if (m) {
      // Prefer the wasm/test one that uses module_registry()
      const all = [...t.matchAll(/build_manifest_json\(([\s\S]*?)\)/g)];
      const preferred = all.find((x) => x[1].includes("module_registry()")) || all[all.length - 1];
      manifestBody = `build_manifest_json(${preferred[1]})`;
    } else {
      manifestBody = `build_manifest_json("${spec.id}", "${spec.label}", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])`;
    }

    // Ensure imports for build_manifest_json at crate root for the pub fn — use a small helper region
    const helper = `
// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json${spec.commands ? ", FlowExtensionCommand" : ""}${spec.settings ? ", FlowExtensionSetting" : ""}};
    ${manifestBody}
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> neural_engine::Registry {
    let mut registry = neural_engine::Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest
`;
    // If module_registry already exists as private fn, remove it and use pub version
    t = t.replace(/\n(?:#\[cfg\([^\]]+\)\]\n)?fn module_registry\(\) -> Registry \{[\s\S]*?\n\}\n/g, "\n");
    // Insert before Tests region or at end before Wasm
    if (t.includes("// #region 🔖️Tests")) {
      t = t.replace("// #region 🔖️Tests", helper + "\n// #region 🔖️Tests");
    } else {
      t = t + helper;
    }
  }

  if (!t.includes("ExtensionGuest")) {
    t = t.trimEnd() + "\n" + buildGuestRegion(spec) + "\n";
  }

  // Fix duplicate module_registry if both private and pub exist — already removed private

  return t;
}

function writePackage(spec) {
  const destRoot = path.join(flowPluginExtCorrect, spec.emojiDir);
  fs.mkdirSync(destRoot, { recursive: true });
  const srcText = fs.readFileSync(spec.src, "utf8");
  const transformed = ensureManifestFn(srcText, spec);
  fs.writeFileSync(path.join(destRoot, "🦀️component.rs"), transformed);

  const rustDir = path.join(destRoot, "📦️packages", "🦀️rust");
  fs.mkdirSync(rustDir, { recursive: true });

  const crateName = `semio-s-plugin-flow-extension-${spec.id}`;
  const cargo = `[package]
name = "${crateName}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Flow ${spec.label} extension — contributes ${spec.id} operators to flow-play and procedural3d-play"

[lints]
workspace = true

[package.metadata.component]
package = "semio:flow-extension-${spec.id}"

[package.metadata.semio]
role = "extension"
extends = "flow"
contributes = ["flow.extension"]

[lib]
crate-type = ["cdylib", "rlib"]
path = "📦️glue.rs"

[features]
default = ["component-guest"]
component-guest = ["dep:semio-framework-plugin", "dep:semio-framework-core"]

[dependencies]
flow_extension_sdk = { path = "../../../../../../../${framework}/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
neural_engine = { path = "../../../../../../../${framework}/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust", package = "semio-framework-os-kernel-neural-engine" }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
semio-framework-plugin = { path = "../../../../../../../${framework}/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", features = ["component-guest"], package = "semio-framework-plugin", optional = true }
semio-framework-core = { path = "../../../../../../../${framework}/📦️packages/🦀️rust", package = "semio-framework-core", optional = true }
`;
  fs.writeFileSync(path.join(rustDir, "Cargo.toml"), cargo);

  fs.writeFileSync(
    path.join(rustDir, "📦️glue.rs"),
    `//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
`,
  );

  fs.writeFileSync(
    path.join(rustDir, "📜️script.ts"),
    `#!/usr/bin/env bun
/** ${spec.emojiDir.charAt(0)} \`@semio-tech/flow-extension-${spec.id}-rust\` router: \`bun ./📜️script.ts test\`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../${framework}/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["${crateName}"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
`,
  );

  fs.writeFileSync(
    path.join(rustDir, "📋️project.json"),
    JSON.stringify(
      {
        name: `@semio-tech/flow-extension-${spec.id}-rust`,
        $schema: "../../../../../../../node_modules/nx/schemas/project-schema.json",
        namedInputs: {
          default: [`{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/️️extensions/${spec.emojiDir}/**/*.rs`, "{projectRoot}/**/*"],
        },
        targets: {
          test: {
            executor: "nx:run-commands",
            options: {
              cwd: `✏️s/🔌️plugins/🌊️flow/️️extensions/${spec.emojiDir}/📦️packages/🦀️rust`,
              command: "bun ./📜️script.ts test",
              forwardAllArgs: true,
            },
          },
          "test-quick": {
            executor: "nx:run-commands",
            options: {
              cwd: `✏️s/🔌️plugins/🌊️flow/️️extensions/${spec.emojiDir}/📦️packages/🦀️rust`,
              command: "bun ./📜️script.ts test quick",
              forwardAllArgs: true,
            },
          },
          "test-long": {
            executor: "nx:run-commands",
            options: {
              cwd: `✏️s/🔌️plugins/🌊️flow/️️extensions/${spec.emojiDir}/📦️packages/🦀️rust`,
              command: "bun ./📜️script.ts test long",
              forwardAllArgs: true,
            },
          },
          "test-exhaustive": {
            executor: "nx:run-commands",
            options: {
              cwd: `✏️s/🔌️plugins/🌊️flow/️️extensions/${spec.emojiDir}/📦️packages/🦀️rust`,
              command: "bun ./📜️script.ts test exhaustive",
              forwardAllArgs: true,
            },
          },
        },
      },
      null,
      2,
    ).replaceAll("️️extensions", fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions"))),
  );

  // delete old framework source file (cut)
  fs.unlinkSync(spec.src);
  // remove empty dir if only that file
  try {
    fs.rmdirSync(path.join(extFw, spec.emojiDir));
  } catch {}

  console.log("migrated", spec.id, "->", destRoot);
}

for (const spec of SIX) writePackage(spec);
console.log("done packages");
