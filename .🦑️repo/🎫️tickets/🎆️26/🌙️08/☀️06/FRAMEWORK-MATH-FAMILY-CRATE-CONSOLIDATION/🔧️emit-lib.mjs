#!/usr/bin/env bun
/** 🔧️ Emits `📦️packages/🦀️rust/📦️lib.rs`'s pure-wiring module tree from the restructured domain folders. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const MATH = join(REPO, "🧰️framework/🔨️modules/🧮️math");
const TICKET = join(REPO, ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/FRAMEWORK-MATH-FAMILY-CRATE-CONSOLIDATION");

const FLAT = [
  ["algebra", "➕️algebra"],
  ["cas", "🧮️cas"],
  ["causal", "🔗️causal"],
  ["entropy", "🎲️entropy"],
  ["fuzzy", "🌫️fuzzy"],
  ["geometry", "📐️geometry"],
  ["lie", "🔷️lie"],
  ["number", "🔢️number"],
  ["optimize", "🎯️optimize"],
  ["polynomial", "📈️polynomial"],
  ["probability", "🎲️probability"],
  ["random", "🎲️random"],
  ["sampling", "🎯️sampling"],
  ["signal", "📶️signal"],
  ["spatial", "🗺️spatial"],
  ["statistics", "📊️statistics"],
  ["tabular", "📋️tabular"],
];

const GRAPH_LEAVES = [
  ["manifest", "🕸️graph/🛂️manifest"],
  ["traversal", "🕸️graph/🚶️traversal"],
  ["operators", "🕸️graph/🔧️operators"],
  ["drawing", "🕸️graph/🖊️drawing"],
  ["dsl", "🕸️graph/🗣️dsl"],
];

const wfcMods = JSON.parse(readFileSync(join(TICKET, "🧪️wfc-modules.json"), "utf8"));
const wfcExports = readFileSync(join(TICKET, "🧪️wfc-exports.txt"), "utf8").trimEnd();

const lines = [];
lines.push('//! 🧮️ The semio math framework: one crate for every mathematical domain the OS kernel, the s-modules and the plugins compute with.');
lines.push('//!');
lines.push('//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.');
lines.push('');
for (const [mod, dir] of FLAT) {
  lines.push(`#[path = "../../${dir}/🦀️component.rs"]`);
  lines.push(`pub mod ${mod};`);
  lines.push('');
}

lines.push('#[path = "."]');
lines.push('pub mod graph {');
lines.push('    #[path = "../../🕸️graph/🦀️component.rs"]');
lines.push('    mod component;');
lines.push('    pub use component::*;');
lines.push('');
for (const [mod, dir] of GRAPH_LEAVES) {
  lines.push(`    #[path = "../../${dir}/🦀️component.rs"]`);
  lines.push(`    pub mod ${mod};`);
  lines.push('');
}
lines.push('    #[path = "."]');
lines.push('    pub mod normal {');
lines.push('        #[path = "../../🕸️graph/➕️normal/↔undirected/🦀️component.rs"]');
lines.push('        pub mod undirected;');
lines.push('');
lines.push('        #[path = "../../🕸️graph/➕️normal/➡️directed/🦀️component.rs"]');
lines.push('        pub mod directed;');
lines.push('    }');
lines.push('');
lines.push('    #[path = "."]');
lines.push('    pub mod ports {');
lines.push('        #[path = "../../🕸️graph/🔌️ports/↔undirected/🦀️component.rs"]');
lines.push('        pub mod undirected;');
lines.push('');
lines.push('        #[path = "."]');
lines.push('        pub mod directed {');
lines.push('            #[path = "../../🕸️graph/🔌️ports/➡️directed/➕️normal/🦀️component.rs"]');
lines.push('            pub mod normal;');
lines.push('        }');
lines.push('    }');
lines.push('}');
lines.push('');

lines.push('#[path = "."]');
lines.push('pub mod wfc {');
for (const m of wfcMods) {
  lines.push(`    #[path = "../../🧩️wfc/${m.folder}/🦀️component.rs"]`);
  lines.push(`    ${m.vis} mod ${m.name};`);
}
lines.push('');
for (const line of wfcExports.split('\n')) lines.push(line ? `    ${line}` : '');
lines.push('}');
lines.push('');

writeFileSync(join(MATH, "📦️packages/🦀️rust/📦️lib.rs"), lines.join('\n'));
console.log(`wrote 📦️lib.rs (${lines.length} lines)`);
