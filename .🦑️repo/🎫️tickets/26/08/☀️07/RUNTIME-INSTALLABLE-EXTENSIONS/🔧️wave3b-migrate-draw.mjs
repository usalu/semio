#!/usr/bin/env bun
/** Wave 3.b — migrate flow 🖍️draw into a packaged extension; Scene APIs → flow core. */
import fs from "fs";
import path from "path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = path.join(ROOT, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");

function findChild(dir, pred) {
  for (const name of fs.readdirSync(dir)) {
    if (pred(name)) return path.join(dir, name);
  }
  throw new Error(`no child matching in ${dir}`);
}

const flowFw = path.join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
const flowExtsFw = findChild(flowFw, (n) => n.includes("extensions"));
const drawOldDir = findChild(flowExtsFw, (n) => n.includes("draw"));
const drawOldComponent = path.join(drawOldDir, "🦀️component.rs");
const coreComponent = path.join(findChild(flowFw, (n) => n.includes("core")), "🦀️component.rs");
const flowGlue = path.join(flowFw, "📦️packages/🦀️rust/📦️glue.rs");
const flowCargo = path.join(flowFw, "📦️packages/🦀️rust/Cargo.toml");

const flowPlugin = path.join(ROOT, "✏️s/🔌️plugins/🌊️flow");
const flowExtsPlugin = findChild(flowPlugin, (n) => n.includes("extensions"));
const bimDir = findChild(flowExtsPlugin, (n) => n.includes("bim"));
const drawNewDir = path.join(flowExtsPlugin, "🖍️draw");
const drawRust = path.join(drawNewDir, "📦️packages/🦀️rust");

console.log("drawOld", drawOldDir);
console.log("drawNew", drawNewDir);
console.log("core", coreComponent);

//#region DrawingKernel region source
const DRAWING_KERNEL_REGION = `
// #region 🖍️DrawingKernel
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use semio_s_2d::{block_on as drawing_block_on, DrawingHandle, DrawingStore};

static DRAWING_KERNEL: LazyLock<Mutex<DrawingStore>> = LazyLock::new(|| Mutex::new(DrawingStore::new()));

fn drawing_kernel() -> &'static Mutex<DrawingStore> {
    &DRAWING_KERNEL
}

/// 🖊️ Runs \`f\` against the process-wide 2D drawing kernel.
pub fn with_drawing_kernel<T>(f: impl FnOnce(&mut DrawingStore) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = drawing_kernel().lock().map_err(|_| EvalError::InvalidInput("draw kernel lock poisoned".into()))?;
    f(&mut guard)
}

/// 🧯️ Internal error type for drawing JSON-bridging helpers.
#[derive(Debug, thiserror::Error)]
enum DrawingKernelError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
}

/// 🧹️ Retains only drawing handles referenced by the current evaluation outputs.
pub fn retain_drawing_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = drawing_kernel().lock() {
        guard.retain_sync(&live_set);
    }
}

/// 🎬️ Flattens a drawing handle to JSON scene payload.
pub fn render_scene_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match drawing_block_on(store.flatten_scene(&drawing)) {
                Ok(scene) => serde_json::to_string(&scene).unwrap_or_else(|_| "{}".into()),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📄️ Exports a drawing handle as SVG JSON wrapper.
pub fn export_svg_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match drawing_block_on(store.export_svg(&drawing)) {
                Ok(svg) => serde_json::json!({ "svg": svg }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📑️ Exports a drawing handle as base64 PDF JSON wrapper.
pub fn export_pdf_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match drawing_block_on(store.export_pdf(&drawing)) {
                Ok(pdf) => serde_json::json!({ "pdf": drawing_base64_encode(&pdf) }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📐️ Exports a drawing handle as base64 DWG JSON wrapper.
pub fn export_dwg_json(handle: &str) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| {
            let drawing = DrawingHandle(handle.to_string());
            match drawing_block_on(store.export_dwg(&drawing)) {
                Ok(dwg) => serde_json::json!({ "dwg": drawing_base64_encode(&dwg) }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            }
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 📐️ Imports a base64 DWG payload into the in-process draw kernel, returning the new drawing handle JSON wrapper.
pub fn import_dwg_json(data_base64: &str) -> String {
    let Ok(bytes) = drawing_base64_decode(data_base64) else {
        return serde_json::json!({ "error": "invalid base64 dwg payload" }).to_string();
    };
    drawing_kernel()
        .lock()
        .ok()
        .map(|mut store| match drawing_block_on(store.import_dwg(&bytes)) {
            Ok(handle) => serde_json::json!({ "handle": handle.as_str() }).to_string(),
            Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.
pub fn dispose_drawing(handle: &str) {
    if let Ok(mut store) = drawing_kernel().lock() {
        drawing_block_on(store.dispose(&DrawingHandle(handle.to_string())));
    }
}

/// 🔍️ Autotraces a bitmap mask into path segments JSON.
pub fn trace_bitmap_json(width: u32, height: u32, mask: &[u8], threshold: f64, simplify_epsilon: f64) -> String {
    drawing_kernel()
        .lock()
        .ok()
        .and_then(|mut store| match drawing_block_on(store.trace_bitmap(width, height, mask, threshold, simplify_epsilon)) {
            Ok(handle) => match drawing_block_on(store.flatten_scene(&handle)) {
                Ok(scene) => {
                    let segments = scene.nodes.into_iter().find_map(|node| if let semio_s_2d::DrawingNode::Path { segments } = node.node { Some(segments) } else { None });
                    segments.map(|segs| serde_json::json!({ "segments": segs }).to_string())
                }
                Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
            },
            Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

/// 🔀️ Boolean-combines two path segment arrays.
pub fn boolean_segments_json(a_json: &str, b_json: &str, operation: &str) -> String {
    let parse = |json: &str| -> Result<Vec<semio_s_2d::PathSegment>, DrawingKernelError> {
        let parsed: serde_json::Value = serde_json::from_str(json)?;
        if let Some(error) = parsed.get("error").and_then(|v| v.as_str()) {
            return Err(DrawingKernelError::Invalid(error.to_string()));
        }
        let segments_value = parsed.get("segments").cloned().ok_or_else(|| DrawingKernelError::Invalid("missing segments".to_string()))?;
        serde_json::from_value(segments_value).map_err(DrawingKernelError::from)
    };
    drawing_kernel()
        .lock()
        .ok()
        .map(|store| match (parse(a_json), parse(b_json)) {
            (Ok(a), Ok(b)) => match drawing_block_on(store.boolean_segments(&a, &b, operation)) {
                Ok(segments) => serde_json::json!({ "segments": segments }).to_string(),
                Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
            },
            (Err(error), _) | (_, Err(error)) => serde_json::json!({ "error": error.to_string() }).to_string(),
        })
        .unwrap_or_else(|| serde_json::json!({ "error": "draw kernel unavailable" }).to_string())
}

fn drawing_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 63) as usize] as char);
        out.push(TABLE[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((triple >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(triple & 63) as usize] as char } else { '=' });
    }
    out
}

fn drawing_base64_decode(data: &str) -> Result<Vec<u8>, DrawingKernelError> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lookup = [255u8; 256];
    for (index, &byte) in TABLE.iter().enumerate() {
        lookup[byte as usize] = index as u8;
    }
    let cleaned: Vec<u8> = data.bytes().filter(|byte| *byte != b'=' && !byte.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(cleaned.len() * 3 / 4);
    for chunk in cleaned.chunks(4) {
        let mut values = [0u8; 4];
        for (index, &byte) in chunk.iter().enumerate() {
            let value = lookup[byte as usize];
            if value == 255 {
                return Err(DrawingKernelError::Invalid("invalid base64 character".to_string()));
            }
            values[index] = value;
        }
        let triple = ((values[0] as u32) << 18) | ((values[1] as u32) << 12) | ((values[2] as u32) << 6) | (values[3] as u32);
        out.push((triple >> 16) as u8);
        if chunk.len() > 2 {
            out.push((triple >> 8) as u8);
        }
        if chunk.len() > 3 {
            out.push(triple as u8);
        }
    }
    Ok(out)
}
// #endregion 🖍️DrawingKernel
`;
//#endregion

//#region Transform extension component
let drawSrc = fs.readFileSync(drawOldComponent, "utf8");

// Drop KERNEL / with_kernel / DrawModuleError — use flow core surface
drawSrc = drawSrc.replace(
  `use semio_s_2d::{block_on, DrawingError, DrawingHandle, DrawingKernel, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2};
use semio_s_2d::DrawingStore;
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static KERNEL: OnceLock<Mutex<DrawingStore>> = OnceLock::new();

fn kernel() -> &'static Mutex<DrawingStore> {
    KERNEL.get_or_init(|| Mutex::new(DrawingStore::new()))
}

// #region ⚠️ Errors
/// 🧯️ Internal error type for the draw module's JSON-bridging helpers (\`boolean_segments_json\`/\`import_dwg_json\` still flatten it to JSON \`{"error"}\` strings, matching prior behaviour byte-for-byte).
#[derive(Debug, thiserror::Error)]
enum DrawModuleError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
}
// #endregion ⚠️ Errors

// #region 🔖️Helpers
fn with_kernel<T>(f: impl FnOnce(&mut DrawingStore) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().lock().map_err(|_| EvalError::InvalidInput("draw kernel lock poisoned".into()))?;
    f(&mut guard)
}
`,
  `use semio_s_2d::{block_on, DrawingError, DrawingHandle, DrawingKernel, DrawingStore, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2};
use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use flow_extension_sdk::with_drawing_kernel as with_kernel;

// #region 🔖️Helpers
`,
);

// module_registry cfg
drawSrc = drawSrc.replace(
  `#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {`,
  `#[cfg(any(test, feature = "component-guest"))]
fn module_registry() -> Registry {`,
);

// Strip Scene region
{
  const start = drawSrc.indexOf("// #region 🔖️Scene");
  const end = drawSrc.indexOf("// #endregion 🔖️Scene");
  if (start < 0 || end < 0) throw new Error("Scene region not found");
  drawSrc = drawSrc.slice(0, start) + drawSrc.slice(end + "// #endregion 🔖️Scene".length).replace(/^\n/, "");
}

// Strip WasmExt region
{
  const start = drawSrc.indexOf("// #region 🔖️WasmExt");
  if (start < 0) throw new Error("WasmExt region not found");
  drawSrc = drawSrc.slice(0, start).replace(/\n+$/, "\n");
}

// Update tests to use flow_extension_sdk Scene APIs
drawSrc = drawSrc.replace(
  `    use super::*;
    use flow_extension_sdk::build_manifest_json;`,
  `    use super::*;
    use flow_extension_sdk::{
        boolean_segments_json, build_manifest_json, dispose_drawing, export_dwg_json, export_pdf_json, export_svg_json, import_dwg_json, render_scene_json,
        retain_drawing_handles, trace_bitmap_json,
    };`,
);

// Append ExtensionGuest (after Tests endregion)
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
            extension_id: "draw".into(),
            label: "Draw".into(),
            icon_id: "draw".into(),
            manifest_json,
        }
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        ExtensionBundle::new("draw", "Draw", "0.1.0")
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

if (!drawSrc.includes("// #endregion 🔖️Tests")) throw new Error("Tests endregion missing");
drawSrc = drawSrc.trimEnd() + "\n" + extensionGuest;

// Add bundle test inside tests module (before closing brace of tests)
const bundleTest = `
    #[test]
    fn bundle_contributes_draw_for_flow_and_procedural3d_play() {
        use flow_extension_sdk::{build_manifest_json, evaluate_json};
        use semio_framework_core::Contribution;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let manifest_json = build_manifest_json("draw", "Draw", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        let bundle = ExtensionBundle::new("draw", "Draw", "0.1.0")
            .extends("flow")
            .contributes(Contribution::FlowExtension {
                app_id: "flow-play".into(),
                extension_id: "draw".into(),
                label: "Draw".into(),
                icon_id: "draw".into(),
                manifest_json: manifest_json.clone(),
            })
            .contributes(Contribution::FlowExtension {
                app_id: "procedural3d-play".into(),
                extension_id: "draw".into(),
                label: "Draw".into(),
                icon_id: "draw".into(),
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
        let installed = install_extension_bundle(bundle).expect("install");
        assert_eq!(installed.contributions.len(), 2);
        assert!(matches!(installed.contributions[0], Contribution::FlowExtension { .. }));
        assert!(matches!(installed.contributions[1], Contribution::FlowExtension { .. }));
        let _ = extension_manifest();
        extension_activate().expect("activate");
        let _ = extension_invoke;
    }
`;

drawSrc = drawSrc.replace(
  `    fn read_point_list_errors_when_entry_is_not_a_point() {
        let list = Dictionary::with_schema("list").insert("0", Value::Atom(Atom::Decimal(1.0)));
        let input = Dictionary::new().insert("points", Value::Dictionary(list));
        assert!(matches!(read_point_list(&input, "points"), Err(EvalError::InvalidInput(_))));
    }
}
// #endregion 🔖️Tests`,
  `    fn read_point_list_errors_when_entry_is_not_a_point() {
        let list = Dictionary::with_schema("list").insert("0", Value::Atom(Atom::Decimal(1.0)));
        let input = Dictionary::new().insert("points", Value::Dictionary(list));
        assert!(matches!(read_point_list(&input, "points"), Err(EvalError::InvalidInput(_))));
    }
${bundleTest}
}
// #endregion 🔖️Tests`,
);
//#endregion

//#region Create extension package tree
fs.mkdirSync(drawRust, { recursive: true });
fs.writeFileSync(path.join(drawNewDir, "🦀️component.rs"), drawSrc);

fs.writeFileSync(
  path.join(drawRust, "📦️glue.rs"),
  `//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️component.rs.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;
`,
);

const bimCargo = fs.readFileSync(path.join(bimDir, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
const drawCargo = bimCargo
  .replaceAll("semio-s-plugin-flow-extension-bim", "semio-s-plugin-flow-extension-draw")
  .replaceAll("flow-extension-bim", "flow-extension-draw")
  .replace("Flow BIM extension — contributes BIM operators to flow-play and procedural3d-play", "Flow draw extension — contributes 2D vector operators to flow-play and procedural3d-play")
  .replace(
    `neural_engine = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust", package = "semio-framework-os-kernel-neural-engine" }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }`,
    `neural_engine = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust", package = "semio-framework-os-kernel-neural-engine" }
semio-s-2d = { path = "../../../../../../🔨️modules/◻2d/📦️packages/🦀️rust" }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }`,
  );
fs.writeFileSync(path.join(drawRust, "Cargo.toml"), drawCargo);

const bimScript = fs.readFileSync(path.join(bimDir, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
fs.writeFileSync(
  path.join(drawRust, "📜️script.ts"),
  bimScript
    .replaceAll("flow-extension-bim-rust", "flow-extension-draw-rust")
    .replaceAll("semio-s-plugin-flow-extension-bim", "semio-s-plugin-flow-extension-draw")
    .replace("🏗️", "🖊️"),
);

const bimProject = fs.readFileSync(path.join(bimDir, "📦️packages/🦀️rust/📋️project.json"), "utf8");
fs.writeFileSync(
  path.join(drawRust, "📋️project.json"),
  bimProject.replaceAll("🏗️bim", "🖍️draw").replaceAll("flow-extension-bim-rust", "flow-extension-draw-rust"),
);
//#endregion

//#region Patch flow core
let core = fs.readFileSync(coreComponent, "utf8");
if (core.includes("// #region 🖍️DrawingKernel")) {
  console.log("DrawingKernel already present — skipping insert");
} else {
  // Insert before WasmSession region
  const wasmSession = core.indexOf("// #region 🔖️WasmSession");
  if (wasmSession < 0) throw new Error("WasmSession region not found");
  core = core.slice(0, wasmSession) + DRAWING_KERNEL_REGION + "\n" + core.slice(wasmSession);
}

core = core.replace(/\n\s*flow_extension_draw::register\(registry\);/, "");
core = core.replaceAll("flow_extension_draw::retain_drawing_handles", "retain_drawing_handles");
core = core.replaceAll("flow_extension_draw::render_scene_json", "render_scene_json");
core = core.replaceAll("flow_extension_draw::export_svg_json", "export_svg_json");
core = core.replaceAll("flow_extension_draw::export_pdf_json", "export_pdf_json");
core = core.replaceAll("flow_extension_draw::export_dwg_json", "export_dwg_json");
core = core.replaceAll("flow_extension_draw::import_dwg_json", "import_dwg_json");
core = core.replaceAll("flow_extension_draw::trace_bitmap_json", "trace_bitmap_json");
core = core.replaceAll("flow_extension_draw::boolean_segments_json", "boolean_segments_json");
// dispose_drawing wasm wrapper collides with the kernel fn name — drop the wrapper; annotate the kernel API.
core = core.replace(
  /#\[cfg\(target_arch = "wasm32"\)\]\n#\[wasm_bindgen\]\npub fn dispose_drawing\(handle: &str\) \{\n    flow_extension_draw::dispose_drawing\(handle\);\n\}\n\n/,
  "",
);
core = core.replace(
  "/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.\npub fn dispose_drawing(handle: &str) {",
  '/// 🗑️ Disposes a drawing handle owned by the in-process draw kernel.\n#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]\npub fn dispose_drawing(handle: &str) {',
);
fs.writeFileSync(coreComponent, core);
//#endregion

//#region Patch flow glue + Cargo
let glue = fs.readFileSync(flowGlue, "utf8");
glue = glue.replace("extern crate self as flow_extension_draw;\n", "");
glue = glue.replace(
  `
  #[path = "../../../🧩️extensions/🖍️draw/🦀️component.rs"]
  pub mod draw;
`,
  "",
);
glue = glue.replace("pub use extensions::draw::*;\n", "");
fs.writeFileSync(flowGlue, glue);

let cargo = fs.readFileSync(flowCargo, "utf8");
if (!cargo.includes("semio-s-2d")) {
  cargo = cargo.replace(
    `semio-s-3d = { path = "../../../../../../../✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust" }`,
    `semio-s-3d = { path = "../../../../../../../✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust" }
semio-s-2d = { path = "../../../../../../../✏️s/🔨️modules/◻2d/📦️packages/🦀️rust" }`,
  );
}
cargo = cargo.replace(
  'description = "OS flow family — core + extensions Shape V2 glue"',
  'description = "OS flow family — core + remaining path-module extensions; draw/bim are packaged plugins"',
);
fs.writeFileSync(flowCargo, cargo);
//#endregion

//#region Patch procedural callers
const procedural2dEngine = path.join(ROOT, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/⚙️engine/🦀️component.rs");
let p2 = fs.readFileSync(procedural2dEngine, "utf8");
p2 = p2.replace("use flow_extension_draw::render_scene_json;", "use flow_core::render_scene_json;");
fs.writeFileSync(procedural2dEngine, p2);

const proceduralCargo = path.join(ROOT, "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml");
let pc = fs.readFileSync(proceduralCargo, "utf8");
pc = pc.replace(
  `flow_extension_draw = { path = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", default-features = false, package = "semio-framework-os-flow" }`,
  `flow_core = { path = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", default-features = false, package = "semio-framework-os-flow" }`,
);
fs.writeFileSync(proceduralCargo, pc);

const proceduralGlue = path.join(ROOT, "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs");
let pg = fs.readFileSync(proceduralGlue, "utf8");
pg = pg.replace("extern crate flow_extension_draw as flow_core;\nextern crate flow_extension_draw as flow_extension_brep;\n", "extern crate flow_core;\nextern crate flow_core as flow_extension_brep;\n");
fs.writeFileSync(proceduralGlue, pg);
//#endregion

//#region Workspace member + policy allowlist
let rootCargo = fs.readFileSync(path.join(ROOT, "Cargo.toml"), "utf8");
const memberLine = `    "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust",`;
if (!rootCargo.includes("🧩️extensions/🖍️draw/📦️packages")) {
  rootCargo = rootCargo.replace(
    `    "✏️s/🔌️plugins/🌊️flow/️️extensions/🏗️bim/📦️packages/🦀️rust",`,
    `    "✏️s/🔌️plugins/🌊️flow/️️extensions/🏗️bim/📦️packages/🦀️rust",\n${memberLine}`,
  );
  // try exact bim path from file
  if (!rootCargo.includes("🧩️extensions/🖍️draw/📦️packages")) {
    const bimMember = rootCargo.split("\n").find((l) => l.includes("flow") && l.includes("bim") && l.includes("extensions"));
    if (!bimMember) throw new Error("bim workspace member not found");
    rootCargo = rootCargo.replace(bimMember, `${bimMember}\n${memberLine}`);
  }
}
// Retarget stale draw alias comment stays; leave alias pointing at os-flow for any residual refs or remove
rootCargo = rootCargo.replace(
  /semio-s-kernel-flow-extension-draw = \{ path = "[^"]+", package = "semio-framework-os-flow" \}/,
  `semio-s-plugin-flow-extension-draw = { path = "✏️s/🔌️plugins/🌊️flow/️️extensions/🖍️draw/📦️packages/🦀️rust" }`,
);
// Fix emoji path if wrong
if (rootCargo.includes('flow/️️extensions/🖍️draw') && !fs.existsSync(path.join(ROOT, "✏️s/🔌️plugins/🌊️flow/️️extensions/🖍️draw/📦️packages/🦀️rust"))) {
  // use the actual emoji folder name from drawNewDir relative
  const rel = path.relative(ROOT, drawRust).split(path.sep).join("/");
  rootCargo = rootCargo.replace(/semio-s-plugin-flow-extension-draw = \{ path = "[^"]+" \}/, `semio-s-plugin-flow-extension-draw = { path = "${rel}" }`);
  rootCargo = rootCargo.replace(/"✏️s\/🔌️plugins\/🌊️flow\/[^"]*draw\/📦️packages\/🦀️rust"/, `"${rel}"`);
}
fs.writeFileSync(path.join(ROOT, "Cargo.toml"), rootCargo);

let scriptTs = fs.readFileSync(path.join(ROOT, "📜️script.ts"), "utf8");
scriptTs = scriptTs.replace(
  `["flow_core", "flow_extension_draw", "flow_extension_brep",`,
  `["flow_core", "flow_extension_brep",`,
);
fs.writeFileSync(path.join(ROOT, "📜️script.ts"), scriptTs);
//#endregion

//#region Delete old draw path-module
fs.rmSync(drawOldDir, { recursive: true, force: true });
console.log("deleted old draw at", drawOldDir);
//#endregion

fs.writeFileSync(
  path.join(TICKET, "wave3b-migrate-log.json"),
  JSON.stringify(
    {
      drawNewDir,
      drawRust,
      drawOldDeleted: drawOldDir,
      corePatched: true,
      gluePatched: true,
    },
    null,
    2,
  ),
);
console.log("done");
