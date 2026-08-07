import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
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

const OLD_PRELUDE = `pub use crate::infinite::board::ports::directed::dag as dag;
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

const NEW_PRELUDE = `use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

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

for (const dir of mods) {
  const p = path.join(FLOW, dir, "🦀️component.rs");
  let text = fs.readFileSync(p, "utf8");
  if (!text.includes(OLD_PRELUDE)) {
    console.error("prelude not found in", dir);
    // try softer replace
    if (text.includes("pub use crate::infinite::board::ports::directed::dag as dag;")) {
      text = text.replace(OLD_PRELUDE, NEW_PRELUDE);
      if (text.includes("pub use crate::infinite::board::ports::directed::dag as dag;")) {
        console.error("FAILED soft replace", dir);
        continue;
      }
    } else {
      continue;
    }
  } else {
    text = text.replace(OLD_PRELUDE, NEW_PRELUDE);
  }
  // Fix remaining bare neural:: / dag:: / canvas:: that aren't crate:: prefixed in use lines — code uses neural::Registry etc.
  // In Rust, neural:: in expression position after `use crate::neural` works as local binding... 
  // Actually `use crate::neural` imports the module, so `neural::Registry` works in the module.
  // `use crate::neural::{...}` also brings items into scope.
  // Good.

  // Fix tests that use `use canvas::` or `use dag::` or `use neural::`
  text = text.replace(/use canvas::/g, "use crate::canvas::");
  text = text.replace(/use dag::/g, "use crate::dag::");
  text = text.replace(/use neural::/g, "use crate::neural::");

  fs.writeFileSync(p, text);
  console.log("fixed prelude", dir);
}

// Verify glue
const glue = path.join(FLOW, "📦️packages", "🦀️rust", "📦️glue.rs");
console.log("--- glue ---");
console.log(fs.readFileSync(glue, "utf8"));

// List flow children
console.log("--- flow children ---");
for (const c of fs.readdirSync(FLOW)) console.log(JSON.stringify(c));

// Spot-check pub(crate) on EvalBridge
const bridge = fs.readFileSync(path.join(FLOW, "🌉️bridge", "🦀️component.rs"), "utf8");
const hits = bridge.split("\n").filter((l) => /EvalBridge|EvalBridgeFn|parse_bridge/.test(l));
console.log("--- bridge pub ---");
hits.slice(0, 30).forEach((l) => console.log(l));

// Spot-check host doesn't redefine FlowCoreError wrongly
const hostHead = fs.readFileSync(path.join(FLOW, "🖥️host", "🦀️component.rs"), "utf8").split("\n").slice(0, 80).join("\n");
console.log("--- host head ---");
console.log(hostHead);
