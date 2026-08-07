import {
  readdirSync,
  readFileSync,
  writeFileSync,
  mkdirSync,
  rmSync,
  cpSync,
  existsSync,
  renameSync,
} from "fs";
import { join, dirname } from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = `${REPO}/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS`;

function findChild(dir, pred) {
  const name = readdirSync(dir).find(pred);
  if (!name) throw new Error(`No child matching in ${dir}`);
  return join(dir, name);
}

function findChildName(dir, pred) {
  const name = readdirSync(dir).find(pred);
  if (!name) throw new Error(`No child name matching in ${dir}`);
  return name;
}

const flowMod = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
const fext = findChild(flowMod, (n) => n.includes("extensions"));
const flowPlugin = join(REPO, "✏️s/🔌️plugins/🌊️flow");
const pext = findChild(flowPlugin, (n) => n.includes("extensions"));
const bimDir = findChild(pext, (n) => n.includes("bim"));
const bimCargo = readFileSync(join(bimDir, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
const bimScript = readFileSync(join(bimDir, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
const bimProject = readFileSync(join(bimDir, "📦️packages/🦀️rust/📋️project.json"), "utf8");
const bimGlue = readFileSync(join(bimDir, "📦️packages/🦀️rust/📦️glue.rs"), "utf8");
const bimComponent = readFileSync(join(bimDir, "🦀️component.rs"), "utf8");
const guestStart = bimComponent.indexOf("// #region 🔖️ExtensionGuest");
const guestEnd = bimComponent.indexOf("// #endregion 🔖️ExtensionGuest") + "// #endregion 🔖️ExtensionGuest".length;
const bimGuestTemplate = bimComponent.slice(guestStart, guestEnd);

const EXTS = [
  { id: "core", title: "Core", icon: "core", pred: (n) => n.includes("core") },
  { id: "math", title: "Math", icon: "math", pred: (n) => n.includes("math") },
  { id: "text", title: "Text", icon: "text", pred: (n) => n.includes("text") },
  { id: "logic", title: "Logic", icon: "logic", pred: (n) => n.includes("logic") },
  { id: "dictionary", title: "Dictionary", icon: "dictionary", pred: (n) => n.includes("dictionary") },
  { id: "list", title: "List", icon: "list", pred: (n) => n.includes("list") },
];

function stripWasmExt(src) {
  const start = src.indexOf("// #region 🔖️WasmExt");
  if (start < 0) return src.replace(/\s*$/, "\n");
  return src.slice(0, start).replace(/\s*$/, "\n");
}

function extractManifestArgs(src, id, title) {
  // Prefer the non-wasm test call; fall back to defaults.
  const re = /build_manifest_json\(([\s\S]*?)\);/g;
  let match;
  let best = null;
  while ((match = re.exec(src))) {
    const args = match[1];
    if (args.includes(`"${id}"`) && !args.includes("kind_id")) {
      best = args.trim();
    }
  }
  if (best) return best;
  return `"${id}", "${title}", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]`;
}

function makeGuest(id, title, icon, manifestArgs) {
  return `// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{build_manifest_json, evaluate_json};
    use semio_framework_core::{Contribution, Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::ExtensionBundle;
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn flow_extension_contribution(app_id: &str, manifest_json: String) -> Contribution {
        Contribution::FlowExtension {
            app_id: app_id.into(),
            extension_id: "${id}".into(),
            label: "${title}".into(),
            icon_id: "${icon}".into(),
            manifest_json,
        }
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json(${manifestArgs});
        ExtensionBundle::new("${id}", "${title}", "0.1.0")
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

function makeCargo(id, title) {
  return `[package]
name = "semio-s-plugin-flow-extension-${id}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Flow ${title} extension — contributes ${id} operators to flow-play and procedural3d-play"

[lints]
workspace = true

[package.metadata.component]
package = "semio:flow-extension-${id}"

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
flow_extension_sdk = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
neural_engine = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust", package = "semio-framework-os-kernel-neural-engine" }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
semio-framework-plugin = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", features = ["component-guest"], package = "semio-framework-plugin", optional = true }
semio-framework-core = { path = "../../../../../../../🧰️framework/📦️packages/🦀️rust", package = "semio-framework-core", optional = true }
`;
}

function makeScript(id) {
  return `#!/usr/bin/env bun
/** 🌊️ \`@semio-tech/flow-extension-${id}-rust\` router: \`bun ./📜️script.ts <test>\`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-flow-extension-${id}"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
`;
}

function makeProject(id, dirName) {
  return `{
  "name": "@semio-tech/flow-extension-${id}-rust",
  "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
  "namedInputs": {
    "default": [
      "{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/️️extensions/${dirName}/**/*.rs",
      "{projectRoot}/**/*"
    ]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "✏️s/🔌️plugins/🌊️flow/️️extensions/${dirName}/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      }
    },
    "test-quick": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "✏️s/🔌️plugins/🌊️flow/️️extensions/${dirName}/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test quick",
        "forwardAllArgs": true
      }
    },
    "test-long": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "✏️s/🔌️plugins/🌊️flow/️️extensions/${dirName}/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test long",
        "forwardAllArgs": true
      }
    },
    "test-exhaustive": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "✏️s/🔌️plugins/🌊️flow/️️extensions/${dirName}/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test exhaustive",
        "forwardAllArgs": true
      }
    }
  }
}
`.replaceAll("️️extensions", readdirSync(flowPlugin).find((n) => n.includes("extensions")));
}

const created = [];
const log = [];

for (const ext of EXTS) {
  const srcDirName = findChildName(fext, ext.pred);
  const srcPath = join(fext, srcDirName, "🦀️component.rs");
  const raw = readFileSync(srcPath, "utf8");
  const withoutWasm = stripWasmExt(raw);
  const manifestArgs = extractManifestArgs(raw, ext.id, ext.title);
  const guest = makeGuest(ext.id, ext.title, ext.icon, manifestArgs);
  const component = withoutWasm.trimEnd() + "\n\n" + guest + "\n";

  const destRoot = join(pext, srcDirName);
  const rustPkg = join(destRoot, "📦️packages", "🦀️rust");
  mkdirSync(rustPkg, { recursive: true });
  writeFileSync(join(destRoot, "🦀️component.rs"), component);
  writeFileSync(join(rustPkg, "📦️glue.rs"), bimGlue);
  writeFileSync(join(rustPkg, "Cargo.toml"), makeCargo(ext.id, ext.title));
  writeFileSync(join(rustPkg, "📜️script.ts"), makeScript(ext.id));
  // project.json — use actual emoji dir name from filesystem
  const project = {
    name: `@semio-tech/flow-extension-${ext.id}-rust`,
    $schema: "../../../../../../../node_modules/nx/schemas/project-schema.json",
    namedInputs: {
      default: [
        `{workspaceRoot}/✏️s/🔌️plugins/🌊️flow/${readdirSync(flowPlugin).find((n) => n.includes("extensions"))}/${srcDirName}/**/*.rs`,
        "{projectRoot}/**/*",
      ],
    },
    targets: Object.fromEntries(
      ["test", "test-quick", "test-long", "test-exhaustive"].map((t) => [
        t,
        {
          executor: "nx:run-commands",
          options: {
            cwd: `✏️s/🔌️plugins/🌊️flow/${readdirSync(flowPlugin).find((n) => n.includes("extensions"))}/${srcDirName}/📦️packages/🦀️rust`,
            command: `bun ./📜️script.ts ${t === "test" ? "test" : t.replace("test-", "test ")}`.replace(
              "test test",
              "test",
            ),
            forwardAllArgs: true,
          },
        },
      ]),
    ),
  };
  // fix test command mapping
  project.targets.test.options.command = "bun ./📜️script.ts test";
  project.targets["test-quick"].options.command = "bun ./📜️script.ts test quick";
  project.targets["test-long"].options.command = "bun ./📜️script.ts test long";
  project.targets["test-exhaustive"].options.command = "bun ./📜️script.ts test exhaustive";
  writeFileSync(join(rustPkg, "📋️project.json"), JSON.stringify(project, null, 2) + "\n");

  created.push({ id: ext.id, dirName: srcDirName, destRoot, rustPkg, manifestArgs });
  log.push(`created ${ext.id} at ${destRoot}`);
  console.log(`created ${ext.id} (${srcDirName})`);
}

writeFileSync(join(TICKET, "wave3a-created.json"), JSON.stringify(created, null, 2));
writeFileSync(join(TICKET, "wave3a-extract-log.txt"), log.join("\n") + "\n");
console.log("DONE", created.length);
