/**
 * Wave 3.c scaffold v2 — cleaner split + package files.
 * Geometry session (KERNEL + pub helpers + side APIs) → flow core path module.
 * Operators + ExtensionBundle → ✏️s/.../️️extensions/📐️brep.
 */
import fs from "fs";
import path from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(REPO, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");

const findChild = (dir, pred) => fs.readdirSync(dir).find((n) => pred(n));

function resolve() {
  const flowRoot = path.join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
  const extRoot = path.join(flowRoot, findChild(flowRoot, (n) => n.includes("extensions")));
  const brepDir = path.join(extRoot, findChild(extRoot, (n) => n.includes("brep")));
  const brepFile = path.join(brepDir, findChild(brepDir, (n) => n.includes("component")));
  const coreDir = path.join(flowRoot, findChild(flowRoot, (n) => /core/.test(n)));
  const coreFile = path.join(coreDir, findChild(coreDir, (n) => n.includes("component")));
  const pkgRust = path.join(flowRoot, "📦️packages", "🦀️rust");
  const glueFile = path.join(pkgRust, findChild(pkgRust, (n) => n.includes("glue")));
  const flowCargo = path.join(pkgRust, "Cargo.toml");

  const pluginFlow = path.join(REPO, "✏️s/🔌️plugins/🌊️flow");
  const pluginExt = path.join(pluginFlow, findChild(pluginFlow, (n) => n.includes("extensions")));
  const bimRoot = path.join(pluginExt, findChild(pluginExt, (n) => n.includes("bim")));
  const bimRust = path.join(bimRoot, "📦️packages", "🦀️rust");

  const s3d = path.join(REPO, "✏️s/🔨️modules", findChild(path.join(REPO, "✏️s/🔨️modules"), (n) => n.includes("3d")));
  return { flowRoot, extRoot, brepDir, brepFile, coreDir, coreFile, glueFile, flowCargo, pluginExt, bimRoot, bimRust, s3d };
}

function extractRegion(src, name) {
  const startRe = `// #region ${name}`;
  const endRe = `// #endregion ${name}`;
  const start = src.indexOf(startRe);
  if (start < 0) throw new Error(`missing region ${name}`);
  const end = src.indexOf(endRe, start);
  if (end < 0) throw new Error(`missing end ${name}`);
  return { start, end: end + endRe.length, text: src.slice(start, end + endRe.length) };
}

function stripRegion(src, name) {
  const r = extractRegion(src, name);
  return src.slice(0, r.start) + src.slice(r.end);
}

/** Make top-level `fn` / `static` / `enum` items `pub` (not methods inside impl). */
function publishItems(regionText) {
  return regionText
    .split("\n")
    .map((line) => {
      if (/^fn [a-z_]/.test(line)) return "pub " + line;
      if (/^static /.test(line)) return "pub " + line;
      if (/^enum /.test(line)) return "pub " + line;
      if (/^struct /.test(line) && !line.includes("pub ")) return "pub " + line;
      // already pub fn retain/tessellate/dispose/export/import — leave
      return line;
    })
    .join("\n");
}

function main() {
  const p = resolve();
  let original = fs.readFileSync(p.brepFile, "utf8");
  // Prefer backup if re-run
  const backup = path.join(TICKET, "brep-original-backup.rs");
  if (fs.existsSync(backup) && fs.statSync(backup).size > 1000) {
    original = fs.readFileSync(backup, "utf8");
    console.log("using backup original");
  } else {
    fs.writeFileSync(backup, original);
    console.log("wrote backup");
  }

  const firstRegion = original.indexOf("// #region");
  const preamble = original.slice(0, firstRegion);
  const helpers = publishItems(extractRegion(original, "🔖️Helpers").text);
  const tessellation = extractRegion(original, "🔖️Tessellation").text; // already pub fns
  const errors = publishItems(extractRegion(original, "⚠️ Errors").text);
  const media = extractRegion(original, "🔖️MediaExport").text; // already pub fns + private bridges

  // Publish private bridge fns inside media/tessellation that extension tests may need
  const tessPub = tessellation.replace(/^fn tessellate_geometry_json_for_wasm/m, "pub fn tessellate_geometry_json_for_wasm");
  const mediaPub = media
    .replace(/^fn export_glb_via_tessellation/m, "pub fn export_glb_via_tessellation")
    .replace(/^fn import_glb_via_tessellation/m, "pub fn import_glb_via_tessellation");

  // Preamble: drop nothing; add pub to statics
  const preamblePub = preamble
    .replace(/^static KERNEL/m, "pub static KERNEL")
    .replace(/^static MESH_CACHE/m, "pub static MESH_CACHE")
    .replace(/^fn kernel\(/m, "pub fn kernel(")
    .replace(/^fn mesh_cache\(/m, "pub fn mesh_cache(")
    .replace(/^fn evict_mesh_cache_for_handles\(/m, "pub fn evict_mesh_cache_for_handles(")
    .replace(/^fn evict_mesh_cache_for_handle\(/m, "pub fn evict_mesh_cache_for_handle(");

  const geometryModule = `//! 📐️ Flow brep geometry session — in-process kernel side APIs.
//!
//! Hosts (procedural3d, playbook, flow wasm exports) call these without depending on the
//! packaged brep operator extension. Operator crates import the same session so handles match
//! when linked into one native/wasm image.

${preamblePub}
${helpers}

${errors}

${tessPub}

${mediaPub}
`;

  const geometryDir = path.join(p.coreDir, "📐️brep-geometry");
  fs.mkdirSync(geometryDir, { recursive: true });
  const geometryPath = path.join(geometryDir, "🦀️component.rs");
  fs.writeFileSync(geometryPath, geometryModule);
  console.log("geometry module lines", geometryModule.split("\n").length);

  // Operators body: from end of Helpers through before Tessellation, plus Tests (stripped wasm)
  let ops = original;
  ops = stripRegion(ops, "🔖️Tessellation");
  ops = stripRegion(ops, "⚠️ Errors");
  ops = stripRegion(ops, "🔖️MediaExport");
  ops = stripRegion(ops, "🔖️WasmExt");
  // Remove preamble+helpers — keep from first operator region
  const helpersEnd = extractRegion(original, "🔖️Helpers").end;
  // Find content after helpers in the stripped ops — use original offset carefully
  // Rebuild: take original after helpers, then strip the four regions from that slice
  let opsBody = original.slice(helpersEnd);
  for (const name of ["🔖️Tessellation", "⚠️ Errors", "🔖️MediaExport", "🔖️WasmExt"]) {
    if (opsBody.includes(`// #region ${name}`)) opsBody = stripRegion(opsBody, name);
  }

  // Relax module_registry cfg so component-guest can build manifest
  opsBody = opsBody.replace(
    "#[cfg(any(test, target_arch = \"wasm32\"))]\nfn module_registry()",
    "fn module_registry()",
  );

  const extensionComponent = `//! 🔷️ Flow brep extension — geometry operators packaged as a runtime-installable unit.

use flow_extension_sdk::brep_geometry::{
    classify_number, decode_base64, dispose_geometry, domain_span, encode_base64, export_solid_json, geometry_channel,
    geometry_dict, import_solid_json, kernel, kind_label, list_channel, list_indices, map_kernel_error, mesh_cache,
    number_channel, number_dictionary, out_curve, out_face, out_solid, out_wire, point_channel, point_dictionary,
    points_to_grid, read_channel_number, read_geometry, read_geometry_list, read_list, read_nested_point_lists,
    read_optional_geometry, read_point_list, read_text, read_xyz, read_xyz_dict, retain_geometry_handles,
    tessellate_geometry, text_dictionary, vector_channel, vector_dictionary, wire_from_points, with_kernel,
    with_kernel_read,
};
use neural_engine::{channel_output, Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use semio_s_3d::brep::engine::{block_on, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use semio_s_3d::brep::kernel::Brep;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, RwLock};

${opsBody.trim()}

// #region 🔖️ExtensionGuest
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
            extension_id: "brep".into(),
            label: "Brep".into(),
            icon_id: "brep".into(),
            manifest_json,
        }
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        ExtensionBundle::new("brep", "Brep", "0.3.0")
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

  // Insert bundle test before endregion Tests
  let finalExt = extensionComponent;
  if (finalExt.includes("// #endregion 🔖️Tests")) {
    const bundleTest = `
    #[test]
    fn extension_bundle_extends_flow_and_evaluates_box() {
        use flow_extension_sdk::{build_manifest_json, evaluate_json};
        use semio_framework_core::Contribution;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let _serial = test_serial();
        reset_test_kernel();
        let manifest_json = build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        let bundle = ExtensionBundle::new("brep", "Brep", "0.3.0")
            .extends("flow")
            .contributes(Contribution::FlowExtension {
                app_id: "flow-play".into(),
                extension_id: "brep".into(),
                label: "Brep".into(),
                icon_id: "brep".into(),
                manifest_json: manifest_json.clone(),
            })
            .contributes(Contribution::FlowExtension {
                app_id: "procedural3d-play".into(),
                extension_id: "brep".into(),
                label: "Brep".into(),
                icon_id: "brep".into(),
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
        assert_eq!(extension_manifest().extension_id, "brep");
        let input = Dictionary::new()
            .insert("width", Value::Dictionary(number_dictionary(1.0)))
            .insert("depth", Value::Dictionary(number_dictionary(1.0)))
            .insert("height", Value::Dictionary(number_dictionary(1.0)));
        let req = serde_json::json!({
            "operatorId": "brep.prim3d.box",
            "inputJson": serde_json::to_string(&input).unwrap(),
            "nodeHash": 1,
        });
        let out: Dictionary = serde_json::from_slice(&extension_invoke("evaluate", req.to_string().as_bytes()).unwrap()).unwrap();
        assert_eq!(channel_payload(&out, "solid").schema(), Some("geometry"));
    }
`;
    finalExt = finalExt.replace("// #endregion 🔖️Tests", `${bundleTest}\n// #endregion 🔖️Tests`);
  }

  const brepExtRoot = path.join(p.pluginExt, "📐️brep");
  const brepRust = path.join(brepExtRoot, "📦️packages", "🦀️rust");
  fs.mkdirSync(brepRust, { recursive: true });
  fs.writeFileSync(path.join(brepExtRoot, "🦀️component.rs"), finalExt);
  console.log("extension component lines", finalExt.split("\n").length);

  // Cargo.toml — handcraft from BIM template
  const bimCargo = fs.readFileSync(path.join(p.bimRust, "Cargo.toml"), "utf8");
  // Relative from extension rust pkg to s3d: ../../../../../../🔨️modules/🌊️3d/📦️packages/🦀️rust
  // extension: ✏️s/🔌️plugins/🌊️flow/️️extensions/📐️brep/📦️packages/🦀️rust
  // s3d:       ✏️s/🔨️modules/🌊️3d/📦️packages/🦀️rust
  // from rust: ../../../../../../ gets to ✏️s/, then 🔨️modules/🌊️3d/...
  const s3dName = path.basename(p.s3d);
  let cargo = bimCargo
    .replaceAll("semio-s-plugin-flow-extension-bim", "semio-s-plugin-flow-extension-brep")
    .replaceAll("semio:flow-extension-bim", "semio:flow-extension-brep")
    .replace("Flow BIM extension — contributes BIM operators to flow-play and procedural3d-play", "Flow Brep extension — contributes B-Rep geometry operators to flow-play and procedural3d-play");

  if (!cargo.includes("semio-s-3d")) {
    cargo = cargo.replace(
      "[dependencies]\n",
      `[dependencies]\nsemio-s-3d = { path = "../../../../../../🔨️modules/${s3dName}/📦️packages/🦀️rust" }\nbase64 = "0.22"\nthiserror = "2"\n`,
    );
  }
  fs.writeFileSync(path.join(brepRust, "Cargo.toml"), cargo);

  fs.writeFileSync(
    path.join(brepRust, "📦️glue.rs"),
    `//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.\n\n#[path = "../../🦀️component.rs"]\nmod component;\npub use component::*;\n`,
  );

  const bimScript = fs.readFileSync(path.join(p.bimRust, findChild(p.bimRust, (n) => n.includes("script"))), "utf8");
  fs.writeFileSync(
    path.join(brepRust, "📜️script.ts"),
    bimScript
      .replaceAll("semio-s-plugin-flow-extension-bim", "semio-s-plugin-flow-extension-brep")
      .replaceAll("flow-extension-bim-rust", "flow-extension-brep-rust")
      .replace("🏗️", "📐️")
      .replace("bim", "brep"),
  );

  const bimProject = fs.readFileSync(path.join(p.bimRust, findChild(p.bimRust, (n) => n.includes("project"))), "utf8");
  fs.writeFileSync(
    path.join(brepRust, "📋️project.json"),
    bimProject
      .replaceAll("@semio-tech/flow-extension-bim-rust", "@semio-tech/flow-extension-brep-rust")
      .replaceAll("🏗️bim", "📐️brep")
      .replaceAll("/bim/", "/brep/")
      .replaceAll("bim", "brep"),
  );

  fs.writeFileSync(
    path.join(TICKET, "wave3c-paths.json"),
    JSON.stringify({ ...p, brepExtRoot, brepRust, geometryPath }, null, 2),
  );
  console.log("done scaffold", { brepExtRoot, geometryPath });
}

main();
