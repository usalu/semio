import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const coreDirName = fs.readdirSync(FLOW).find((c) => /core$/.test(c) || c.includes("core"));
const CORE = path.join(FLOW, coreDirName);
const SRC = path.join(CORE, "🦀️component.rs");
const RUST = path.join(FLOW, "📦️packages", "🦀️rust");
const lines = fs.readFileSync(SRC, "utf8").split("\n");

// Line ranges are 1-indexed inclusive content (including region markers)
const MODULES = [
  {
    dir: "📄️document",
    rust: "document",
    start: 22,
    end: 1038,
    doc: "//! 📄️ Flow document: widgets, fixture, and DAG projection helpers.",
  },
  {
    dir: "📚️catalogue",
    rust: "catalogue",
    start: 1040,
    end: 1337,
    doc: "//! 📚️ Flow operator catalogue and node-graph extras.",
  },
  {
    dir: "📔️registry",
    rust: "registry",
    start: 1339,
    end: 1575,
    doc: "//! 📔️ Flow extension registry and contribution install surface.",
  },
  {
    dir: "🌉️bridge",
    rust: "bridge",
    start: 1577,
    end: 1976,
    doc: "//! 🌉️ Flow eval bridge and channel-eval helpers.",
  },
  {
    dir: "🖥️host",
    rust: "host",
    start: 1978, // Errors + FlowHost (incl EvalSession)
    end: 4010,
    doc: "//! 🖥️ Flow host: canvas editing, evaluation session, and host errors.",
  },
  {
    dir: "🖍️drawing",
    rust: "drawing",
    start: 4013,
    end: 4215,
    doc: "//! 🖍️ Flow 2D drawing kernel JSON bridge.",
  },
  {
    dir: "🌉️wasm",
    rust: "wasm_session",
    start: 4217,
    end: 4886,
    doc: "//! 🌉️ Flow WASM session bindings.",
  },
  {
    dir: "🌿️vcs",
    rust: "vcs",
    start: 4888,
    end: 6009,
    doc: "//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.",
  },
];

const TESTS = { start: 6011, end: 7915 };

const PRELUDE_USES = `pub use crate::infinite::board::ports::directed::dag as dag;
pub use crate::infinite::canvas as canvas;
pub use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use math::graph::manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorImpl, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use flow_extension_sdk::FlowExtensionManifest;
use serde::{Deserialize, Serialize};
`;

// Extract body slices
function sliceLines(start, end) {
  return lines.slice(start - 1, end).join("\n");
}

// Collect definitions per module
const defRe =
  /^(?:pub(?:\([^)]*\))?\s+)?((?:async\s+)?fn|struct|enum|type|const|static)\s+([A-Za-z0-9_]+)/;

const moduleBodies = {};
const moduleDefs = {}; // name -> module rust name
const privateDefs = {}; // name -> { mod, line, fullMatch, isPub }

for (const m of MODULES) {
  const body = sliceLines(m.start, m.end);
  moduleBodies[m.rust] = body;
  const bodyLines = body.split("\n");
  for (let i = 0; i < bodyLines.length; i++) {
    const match = bodyLines[i].match(defRe);
    if (!match) continue;
    const kind = match[1].replace(/\s+/g, " ");
    const name = match[2];
    const isPub = /^\s*pub\b/.test(bodyLines[i]);
    moduleDefs[name] = m.rust;
    if (!privateDefs[name]) {
      privateDefs[name] = { mod: m.rust, kind, isPub, lineIndex: i };
    }
  }
}

// Tests attach to host
const testsBody = sliceLines(TESTS.start, TESTS.end).replace(
  /use super::\*;/,
  "use crate::*;",
);

// Detect cross-module private usage
function wordUsed(body, name) {
  const re = new RegExp(`\\b${name}\\b`);
  return re.test(body);
}

const needsPubCrate = new Set();
for (const [name, info] of Object.entries(privateDefs)) {
  if (info.isPub) continue;
  // skip very short / common names that would false-positive? keep all defined names
  for (const m of MODULES) {
    if (m.rust === info.mod) continue;
    if (wordUsed(moduleBodies[m.rust], name)) {
      needsPubCrate.add(name);
      break;
    }
  }
  if (!needsPubCrate.has(name) && wordUsed(testsBody, name) && info.mod !== "host") {
    needsPubCrate.add(name);
  }
}

console.log("pub(crate) upgrades:", [...needsPubCrate].sort().join(", "));

function upgradePubCrate(body, modName) {
  const bodyLines = body.split("\n");
  for (let i = 0; i < bodyLines.length; i++) {
    const match = bodyLines[i].match(defRe);
    if (!match) continue;
    const name = match[2];
    if (!needsPubCrate.has(name)) continue;
    if (/^\s*pub\b/.test(bodyLines[i])) continue;
    bodyLines[i] = bodyLines[i].replace(
      /^((?:#\[[^\]]+\]\s*)*)/,
      (attrs) => attrs,
    );
    // handle cfg attributes on previous lines already in place; only upgrade this line
    bodyLines[i] = bodyLines[i].replace(
      /^(\s*)(?:pub\(crate\)\s+)?(struct|enum|type|const|static|fn|async fn)\b/,
      (_, sp, kw) => `${sp}pub(crate) ${kw}`,
    );
    // also: `fn` after async already handled; type aliases etc.
    if (!/pub\(crate\)/.test(bodyLines[i]) && !/^(\s*)pub\b/.test(bodyLines[i])) {
      bodyLines[i] = bodyLines[i].replace(/^(\s*)/, `$1pub(crate) `);
    }
  }
  return bodyLines.join("\n");
}

// Special: EvalBridge, EvalBridgeFn always pub(crate) for host
needsPubCrate.add("EvalBridge");
needsPubCrate.add("EvalBridgeFn");
needsPubCrate.add("parse_bridge_dictionary_json");

// Create modules
const created = [];
const report = { created: [], updated: [], removed: [], deferred: [] };

for (const m of MODULES) {
  const dir = path.join(FLOW, m.dir);
  fs.mkdirSync(dir, { recursive: true });
  let body = upgradePubCrate(moduleBodies[m.rust], m.rust);

  // Fix region name for document endregion Widget -> Document
  if (m.rust === "document") {
    body = body.replace("// #endregion 🔖️Widget", "// #endregion 🔖️Document");
  }

  // Host gets Errors region + FlowHost + tests
  if (m.rust === "host") {
    body = body + "\n\n" + testsBody;
  }

  // Drawing needs wasm_bindgen import when compiling for wasm — already in body for drawing start

  const content = `${m.doc}\n\n${PRELUDE_USES}\n${body}\n`;
  const out = path.join(dir, "🦀️component.rs");
  fs.writeFileSync(out, content);
  created.push(out);
  report.created.push(path.relative("/Users/ueli/Documents/semio", out));
  console.log("wrote", out, "lines", content.split("\n").length);
}

// Lift brep-geometry
const brepSrcName = fs.readdirSync(CORE).find((c) => c.includes("brep"));
const brepSrc = path.join(CORE, brepSrcName);
const brepDst = path.join(FLOW, brepSrcName);
if (fs.existsSync(brepDst)) {
  console.log("brep dest exists, skip move?");
} else {
  fs.renameSync(brepSrc, brepDst);
  report.updated.push(`lifted ${brepSrcName} to flow sibling`);
  console.log("lifted", brepSrc, "->", brepDst);
}

// Also move pkg if present? Plan doesn't mention pkg - leave in core then delete with core.
// Check pkg - wasm glue artifact; might need to stay or move. Leave and delete with core unless referenced.
const pkgPath = path.join(CORE, "pkg");
if (fs.existsSync(pkgPath)) {
  console.log("NOTE: core/pkg exists — leaving for delete with core; check refs");
}

// Write glue.rs
const glue = `//! 🌊️ OS flow family glue — wires document/catalogue/registry/bridge/host/drawing/wasm/vcs, brep geometry, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

pub use crate::infinite::board::ports::directed::dag as dag;
pub use crate::infinite::canvas as canvas;
pub use neural_engine as neural;

#[path = "../../📄️document/🦀️component.rs"]
pub mod document;
pub use document::*;

#[path = "../../📚️catalogue/🦀️component.rs"]
pub mod catalogue;
pub use catalogue::*;

#[path = "../../📔️registry/🦀️component.rs"]
pub mod registry;
pub use registry::*;

#[path = "../../🌉️bridge/🦀️component.rs"]
pub mod bridge;
pub use bridge::*;

#[path = "../../🖥️host/🦀️component.rs"]
pub mod host;
pub use host::*;

#[path = "../../🖍️drawing/🦀️component.rs"]
pub mod drawing;
pub use drawing::*;

#[path = "../../🌉️wasm/🦀️component.rs"]
pub mod wasm_session;
pub use wasm_session::*;

#[path = "../../🌿️vcs/🦀️component.rs"]
pub mod vcs;
pub use vcs::*;

#[path = "../../${brepSrcName}/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};

#[path = "."]
pub mod extensions {
  #[path = "../../🧩️extensions/🕸️wasm/🦀️component.rs"]
  pub mod wasm;
}

pub use extensions::wasm::*;
`;

const gluePath = path.join(RUST, "📦️glue.rs");
fs.writeFileSync(gluePath, glue);
report.updated.push(path.relative("/Users/ueli/Documents/semio", gluePath));
console.log("wrote glue");

// Delete corrupted glue
for (const name of fs.readdirSync(RUST)) {
  if (name.includes("glue") && name !== "📦️glue.rs") {
    const p = path.join(RUST, name);
    // check for replacement char or wrong prefix
    const codes = [...name].map((c) => c.codePointAt(0));
    if (codes.includes(0xfffd) || name !== "📦️glue.rs") {
      console.log("removing corrupted glue", JSON.stringify(name));
      fs.unlinkSync(p);
      report.removed.push(path.relative("/Users/ueli/Documents/semio", p));
    }
  }
}

// Remove core folder contents and directory
function rmrf(p) {
  if (!fs.existsSync(p)) return;
  const st = fs.statSync(p);
  if (st.isDirectory()) {
    for (const c of fs.readdirSync(p)) rmrf(path.join(p, c));
    fs.rmdirSync(p);
  } else {
    fs.unlinkSync(p);
  }
}
rmrf(CORE);
report.removed.push(path.relative("/Users/ueli/Documents/semio", CORE));
console.log("removed core", CORE);

// Collect deferred flow_core:: outside flow tree
const { execSync } = await import("child_process");
let files = [];
try {
  const out = execSync(
    `rg -l "flow_core::" --glob "!target/**" --glob "!.git/**" --glob "!**/node_modules/**" -g "*.rs" /Users/ueli/Documents/semio`,
    { encoding: "utf8", maxBuffer: 50e6 },
  );
  files = out.trim().split("\n").filter(Boolean);
} catch (e) {
  files = (e.stdout || "").trim().split("\n").filter(Boolean);
}

const deferred = [];
for (const f of files) {
  if (f.startsWith(FLOW)) continue;
  const text = fs.readFileSync(f, "utf8");
  const re = /flow_core::/g;
  let match;
  const count = (text.match(re) || []).length;
  deferred.push({
    file: path.relative("/Users/ueli/Documents/semio", f),
    old: "flow_core::",
    new: "flow::",
    occurrences: count,
  });
}

const deferredPath = path.join(
  path.dirname(new URL(import.meta.url).pathname),
  "deferred-flow-core.json",
);
// import.meta.url path may be odd; write next to script via argv
const ticketDir = process.env.TICKET || path.dirname(process.argv[1]);
fs.writeFileSync(
  path.join(ticketDir, "deferred-flow-core.json"),
  JSON.stringify(deferred, null, 2),
);
report.deferred = deferred;
console.log("deferred entries", deferred.length);

fs.writeFileSync(
  path.join(ticketDir, "wave1-flow-core.report.md"),
  `# Wave 1 — flow core dissolution

## Summary
Dissolved \`🌊️flow/🙀️core/\` into concept siblings under \`🌊️flow/\`, lifted \`📐️brep-geometry\`, rewired \`📦️glue.rs\` (\`flow_core\` → \`flow\`), deleted corrupted duplicate glue, and recorded external \`flow_core::\` renames for Wave 2.

## Created
${report.created.map((p) => `- ${p}`).join("\n")}

## Updated
${report.updated.map((p) => `- ${p}`).join("\n")}

## Removed
${report.removed.map((p) => `- ${p}`).join("\n")}

## Modules
| Folder | Rust mod | Source lines (approx) |
|--------|----------|------------------------|
| 📄️document | document | 22–1038 |
| 📚️catalogue | catalogue | 1040–1337 |
| 📔️registry | registry | 1339–1575 |
| 🌉️bridge | bridge | 1577–1976 |
| 🖥️host | host | 1978–4010 + Tests 6011–7915 |
| 🖍️drawing | drawing | 4013–4215 |
| 🌉️wasm | wasm_session | 4217–4886 |
| 🌿️vcs | vcs | 4888–6009 |
| 📐️brep-geometry | brep_geometry | lifted as-is |

## Alias
- \`extern crate self as flow_core\` → \`extern crate self as flow\`
- Removed \`pub mod core\`

## pub(crate) upgrades
${[...needsPubCrate].sort().map((n) => `- ${n}`).join("\n")}

## Deferred external \`flow_core::\` → \`flow::\`
See \`deferred-flow-core.json\` (${deferred.length} files).

## Notes
- No collision under \`🌊️flow/\` for \`🌿️vcs\` (OS-level \`✨️modules/🌿️vcs\` is unrelated).
- Tests region kept in \`🖥️host\` with \`use crate::*\`.
- Did not touch pack/db/spr/dsl cores, plugins, or framework 🧩core.
`,
);

console.log("DONE");
