import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const RUST = path.join(FLOW, "📦️packages", "🦀️rust");

// Restore glue without crate::infinite (wasn't in original glue)
const glue = `//! 🌊️ OS flow family glue — wires document/catalogue/registry/bridge/host/drawing/wasm/vcs, brep geometry, and wasm SDK.
//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.

extern crate self as flow;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

//#region 🔖️KernelModuleAliases
/// 🧬️ Components still use former kernel path names (\`crate::os_store\` / \`os_dsl\` / \`os_spr\`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_vcs;
pub use semio_framework_os_kernel::os_pack;
//#endregion 🔖️KernelModuleAliases

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

#[path = "../../📐️brep-geometry/🦀️component.rs"]
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
fs.writeFileSync(path.join(RUST, "📦️glue.rs"), glue);
console.log("rewrote glue with kernel aliases");

// Restore original-style prelude in each module (local dag/neural aliases like former core)
const OLD = `use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use crate::dag;
use crate::dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use crate::canvas;
use crate::neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorImpl, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use crate::neural;
use math::graph::manifest::{PropertyBag, PropertyValue};
use flow_extension_sdk::FlowExtensionManifest;
use serde::{Deserialize, Serialize};
`;

const NEW = `pub use crate::infinite::board::ports::directed::dag as dag;
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

const mods = [
  "📄️document",
  "📚️catalogue",
  "📔️registry",
  "🌉️bridge",
  "🖥️host",
  "🖍️drawing",
  "🌉️wasm",
  "🌿️vcs",
];

for (const dir of mods) {
  const p = path.join(FLOW, dir, "🦀️component.rs");
  let text = fs.readFileSync(p, "utf8");
  if (!text.includes(OLD)) {
    console.error("OLD prelude not found", dir);
    // show first 30 lines
    console.log(text.split("\n").slice(0, 30).join("\n"));
    continue;
  }
  text = text.replace(OLD, NEW);
  // revert test import rewrites that assumed crate::canvas
  text = text.replace(/use crate::canvas::/g, "use canvas::");
  text = text.replace(/use crate::dag::/g, "use dag::");
  text = text.replace(/use crate::neural::/g, "use neural::");
  fs.writeFileSync(p, text);
  console.log("restored prelude", dir);
}

// Wire infinite as dependency + alias so crate::infinite works (clean fix matching infinite's own pattern)
let cargo = fs.readFileSync(path.join(RUST, "Cargo.toml"), "utf8");
if (!cargo.includes("semio-framework-os-infinite")) {
  cargo = cargo.replace(
    `semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel" }`,
    `semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
semio-framework-os-infinite = { path = "../../../♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }`,
  );
  fs.writeFileSync(path.join(RUST, "Cargo.toml"), cargo);
  console.log("added infinite dep");
}

// Add infinite alias to glue
let glue2 = fs.readFileSync(path.join(RUST, "📦️glue.rs"), "utf8");
if (!glue2.includes("as infinite")) {
  glue2 = glue2.replace(
    "//#endregion 🔖️KernelModuleAliases\n",
    `//#endregion 🔖️KernelModuleAliases

//#region 🔖️InfiniteAlias
/// ♾️ Flow components use \`crate::infinite::{board,canvas}\` paths.
pub use semio_framework_os_infinite as infinite;
//#endregion 🔖️InfiniteAlias
`,
  );
  fs.writeFileSync(path.join(RUST, "📦️glue.rs"), glue2);
  console.log("added infinite alias");
}

console.log("--- glue ---");
console.log(fs.readFileSync(path.join(RUST, "📦️glue.rs"), "utf8"));
