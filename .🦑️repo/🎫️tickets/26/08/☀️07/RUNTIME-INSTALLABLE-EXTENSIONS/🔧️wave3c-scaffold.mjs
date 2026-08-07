/**
 * Wave 3.c — split flow brep into:
 * 1) flow core geometry session (KERNEL + helpers + side APIs)
 * 2) packaged extension (operators + ExtensionBundle)
 *
 * Also scaffolds extension package files and applies surgical glue/caller edits.
 */
import fs from "fs";
import path from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(REPO, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");

function findChild(dir, pred) {
  return fs.readdirSync(dir).find((n) => pred(n));
}

function resolveFlowPaths() {
  const flowRoot = path.join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
  const extName = findChild(flowRoot, (n) => n.includes("extensions"));
  const extRoot = path.join(flowRoot, extName);
  const brepName = findChild(extRoot, (n) => n.includes("brep"));
  const brepDir = path.join(extRoot, brepName);
  const brepFile = path.join(brepDir, findChild(brepDir, (n) => n.includes("component")));
  const coreName = findChild(flowRoot, (n) => /core/.test(n));
  const coreDir = path.join(flowRoot, coreName);
  const coreFile = path.join(coreDir, findChild(coreDir, (n) => n.includes("component")));
  const pkgRust = path.join(flowRoot, "📦️packages", "🦀️rust");
  const glueFile = path.join(pkgRust, findChild(pkgRust, (n) => n.includes("glue")));
  return { flowRoot, extRoot, brepDir, brepFile, coreDir, coreFile, glueFile, pkgRust };
}

function resolveBimPaths() {
  const pluginFlow = path.join(REPO, "✏️s/🔌️plugins/🌊️flow");
  const extName = findChild(pluginFlow, (n) => n.includes("extensions"));
  const bimName = findChild(path.join(pluginFlow, extName), (n) => n.includes("bim"));
  const bimRoot = path.join(pluginFlow, extName, bimName);
  const bimRust = path.join(bimRoot, "📦️packages", "🦀️rust");
  return { pluginFlow, extName, bimRoot, bimRust, pluginExtRoot: path.join(pluginFlow, extName) };
}

function extractRegion(src, name) {
  const start = src.indexOf(`// #region ${name}`);
  if (start < 0) throw new Error(`missing region ${name}`);
  const endTag = `// #endregion ${name}`;
  const end = src.indexOf(endTag, start);
  if (end < 0) throw new Error(`missing end region ${name}`);
  return src.slice(start, end + endTag.length);
}

function stripRegion(src, name) {
  const start = src.indexOf(`// #region ${name}`);
  if (start < 0) return src;
  const endTag = `// #endregion ${name}`;
  const end = src.indexOf(endTag, start);
  if (end < 0) throw new Error(`missing end region ${name}`);
  return src.slice(0, start) + src.slice(end + endTag.length);
}

function main() {
  const flow = resolveFlowPaths();
  const bim = resolveBimPaths();
  const original = fs.readFileSync(flow.brepFile, "utf8");
  fs.writeFileSync(path.join(TICKET, "brep-original-backup.rs"), original);
  console.log("backed up original", original.length, "bytes");

  // --- Build geometry session module for flow core ---
  const helpers = extractRegion(original, "🔖️Helpers");
  const tessellation = extractRegion(original, "🔖️Tessellation");
  const errors = extractRegion(original, "⚠️ Errors");
  const media = extractRegion(original, "🔖️MediaExport");

  // Preamble: everything before first region (statics + imports)
  const firstRegion = original.indexOf("// #region");
  const preamble = original.slice(0, firstRegion);

  const geometryModule = `//! 📐️ Flow brep geometry session — in-process kernel side APIs shared by hosts and the brep extension.
//!
//! Side APIs used outside the operator registry (\`tessellate_geometry\`, \`export_solid_json\`,
//! \`import_solid_json\`, \`retain_geometry_handles\`, \`dispose_geometry\`) live here on the flow
//! core surface so callers do not depend on the packaged brep extension.

${preamble}
${helpers}

${errors}

${tessellation}

${media}
`;

  const geometryPath = path.join(flow.coreDir, "📐️brep-geometry", "🦀️component.rs");
  fs.mkdirSync(path.dirname(geometryPath), { recursive: true });
  fs.writeFileSync(geometryPath, geometryModule);
  console.log("wrote geometry module", geometryPath, geometryModule.split("\n").length, "lines");

  // --- Build extension operator module ---
  let ops = original;
  ops = stripRegion(ops, "🔖️Tessellation");
  ops = stripRegion(ops, "⚠️ Errors");
  ops = stripRegion(ops, "🔖️MediaExport");
  ops = stripRegion(ops, "🔖️WasmExt");

  // Replace preamble imports/statics with imports from flow geometry session via sdk re-exports.
  // Operators still need local access to helpers — they will `use` pub items from flow_extension_sdk.
  // For the split: keep Helpers region in BOTH places initially is wrong.
  // Extension should import pub helpers from sdk instead of defining them.

  // Rebuild ops file: drop preamble statics/helpers; import from sdk.
  const afterHelpers = original.indexOf("// #endregion 🔖️Helpers") + "// #endregion 🔖️Helpers".length;
  let opsBody = original.slice(afterHelpers);
  opsBody = stripRegion(opsBody, "🔖️Tessellation");
  opsBody = stripRegion(opsBody, "⚠️ Errors");
  opsBody = stripRegion(opsBody, "🔖️MediaExport");
  opsBody = stripRegion(opsBody, "🔖️WasmExt");

  // Make helpers/kernel APIs available: use flow_extension_sdk which re-exports from core.
  // Tests reference kernel()/mesh_cache()/reset — those need to stay reachable.
  const extensionComponent = `//! 🔷️ Flow brep extension: native geometry operators (packaged, runtime-installable).

use base64::Engine;
use flow_extension_sdk::brep_geometry::{
    decode_base64, dispose_geometry, encode_base64, export_solid_json, geometry_channel, geometry_dict, import_solid_json, kind_label, list_channel, list_indices, map_kernel_error, number_channel, number_dictionary, out_curve, out_face, out_solid, out_wire, point_channel, point_dictionary, read_channel_number, read_geometry, read_geometry_list, read_list, read_nested_point_lists, read_optional_geometry, read_point_list, read_text, read_xyz, read_xyz_dict, retain_geometry_handles, tessellate_geometry, text_dictionary, vector_channel, vector_dictionary, wire_from_points, with_kernel, with_kernel_read, classify_number, domain_span, points_to_grid, kernel, mesh_cache, evict_mesh_cache_for_handle, evict_mesh_cache_for_handles,
};
use semio_s_3d::brep::engine::{block_on, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use semio_s_3d::brep::kernel::Brep;
use neural_engine::{channel_output, Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, RwLock};

${opsBody}
`;

  // Fix tests that call kernel()/mesh_cache directly — already imported.
  // Fix test that references tessellate_geometry/retain — imported.

  // Add ExtensionGuest + bundle test like BIM at end (before last line)
  const extensionGuest = `
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

  // Insert bundle test into tests module if present
  let finalExt = extensionComponent;
  if (finalExt.includes("// #endregion 🔖️Tests")) {
    const bundleTest = `
    #[test]
    fn extension_bundle_extends_flow_and_evaluates_box() {
        use semio_framework_core::Contribution;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

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
        let installed = extension_manifest();
        assert_eq!(installed.extension_id, "brep");
        assert_eq!(installed.extends, "flow");
        assert_eq!(installed.contributions.len(), 2);
        let input = Dictionary::new()
            .insert("width", Value::Dictionary(number_dictionary(1.0)))
            .insert("depth", Value::Dictionary(number_dictionary(1.0)))
            .insert("height", Value::Dictionary(number_dictionary(1.0)));
        let req = serde_json::json!({
            "operatorId": "brep.prim3d.box",
            "inputJson": serde_json::to_string(&input).unwrap(),
            "nodeHash": 1,
        });
        let out_bytes = extension_invoke("evaluate", req.to_string().as_bytes()).unwrap();
        let out: Dictionary = serde_json::from_slice(&out_bytes).unwrap();
        assert_eq!(channel_payload(&out, "solid").schema(), Some("geometry"));
    }
`;
    finalExt = finalExt.replace("// #endregion 🔖️Tests", bundleTest + "\n// #endregion 🔖️Tests");
  }
  finalExt = finalExt.trimEnd() + "\n" + extensionGuest + "\n";

  // Scaffold extension directory next to bim
  const brepExtRoot = path.join(bim.pluginExtRoot, "📐️brep");
  const brepRust = path.join(brepExtRoot, "📦️packages", "🦀️rust");
  fs.mkdirSync(brepRust, { recursive: true });
  fs.writeFileSync(path.join(brepExtRoot, "🦀️component.rs"), finalExt);
  console.log("wrote extension component", finalExt.split("\n").length, "lines");

  // Package files from BIM templates
  const bimCargo = fs.readFileSync(path.join(bim.bimRust, "Cargo.toml"), "utf8");
  const brepCargo = bimCargo
    .replaceAll("semio-s-plugin-flow-extension-bim", "semio-s-plugin-flow-extension-brep")
    .replaceAll("flow-extension-bim", "flow-extension-brep")
    .replaceAll("Flow BIM extension", "Flow Brep extension")
    .replaceAll("BIM operators", "Brep geometry operators")
    .replace(
      "[dependencies]",
      `[dependencies]
semio-s-3d = { path = "../../../../../../🔨️modules/🌊️3d/📦️packages/🦀️rust" }
base64 = "0.22"
thiserror = "2"`,
    );
  // Fix path depth - BIM has 7 levels of ../ to framework; brep is same depth as BIM
  // BIM: ../../../../../../../🧰️framework/...
  // Add semio-s-3d path carefully - from ✏️s/🔌️plugins/🌊️flow/️️extensions/📐️brep/📦️packages/🦀️rust
  // to ✏️s/🔨️modules/🌊️3d/📦️packages/🦀️rust = ../../../../../../🔨️modules/🌊️3d/📦️packages/🦀️rust
  fs.writeFileSync(path.join(brepRust, "Cargo.toml"), brepCargo);

  const bimGlue = fs.readFileSync(path.join(bim.bimRust, findChild(bim.bimRust, (n) => n.includes("glue"))), "utf8");
  fs.writeFileSync(path.join(brepRust, "📦️glue.rs"), bimGlue);

  const bimScript = fs.readFileSync(path.join(bim.bimRust, findChild(bim.bimRust, (n) => n.includes("script"))), "utf8");
  fs.writeFileSync(
    path.join(brepRust, "📜️script.ts"),
    bimScript.replaceAll("bim", "brep").replaceAll("Bim", "Brep").replaceAll("🏗️", "📐️"),
  );

  const bimProject = fs.readFileSync(path.join(bim.bimRust, findChild(bim.bimRust, (n) => n.includes("project"))), "utf8");
  fs.writeFileSync(
    path.join(brepRust, "📋️project.json"),
    bimProject.replaceAll("bim", "brep").replaceAll("🏗️bim", "📐️brep").replaceAll("@semio-tech/flow-extension-bim-rust", "@semio-tech/flow-extension-brep-rust"),
  );

  fs.writeFileSync(path.join(TICKET, "brep-ext.path"), brepExtRoot + "\n");
  fs.writeFileSync(path.join(TICKET, "brep-geometry.path"), geometryPath + "\n");
  console.log("scaffolded extension at", brepExtRoot);

  // Save paths for follow-up patches
  fs.writeFileSync(
    path.join(TICKET, "wave3c-paths.json"),
    JSON.stringify({ ...flow, ...bim, brepExtRoot, brepRust, geometryPath }, null, 2),
  );
}

main();
