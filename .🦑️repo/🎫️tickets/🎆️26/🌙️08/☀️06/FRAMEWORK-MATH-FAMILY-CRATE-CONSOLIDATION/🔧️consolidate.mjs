#!/usr/bin/env bun
/** 🔧️ One-shot restructure of the math family's 51 crates into the single `semio-framework-math` crate. */
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const MATH = join(REPO, "🧰️framework/🔨️modules/🧮️math");

/** 🗺️ Surviving crate -> destination component file + module path inside the merged crate. */
const SURVIVORS = [
  { old: "➕️algebra/⚡️implementations/🦀️rust", dest: "➕️algebra/🦀️component.rs", mod: "algebra", lib: "mathematical_algebra" },
  { old: "🌫️fuzzy/⚡️implementations/🦀️rust", dest: "🌫️fuzzy/🦀️component.rs", mod: "fuzzy", lib: "mathematical_fuzzy" },
  { old: "🎯️optimize/⚡️implementations/🦀️rust", dest: "🎯️optimize/🦀️component.rs", mod: "optimize", lib: "mathematical_optimize" },
  { old: "🎯️sampling/⚡️implementations/🦀️rust", dest: "🎯️sampling/🦀️component.rs", mod: "sampling", lib: "mathematical_sampling" },
  { old: "🎲️entropy/⚡️implementations/🦀️rust", dest: "🎲️entropy/🦀️component.rs", mod: "entropy", lib: "mathematical_entropy" },
  { old: "🎲️probability/⚡️implementations/🦀️rust", dest: "🎲️probability/🦀️component.rs", mod: "probability", lib: "mathematical_probability" },
  { old: "🎲️random/⚡️implementations/🦀️rust", dest: "🎲️random/🦀️component.rs", mod: "random", lib: "mathematical_random" },
  { old: "📈️polynomial/⚡️implementations/🦀️rust", dest: "📈️polynomial/🦀️component.rs", mod: "polynomial", lib: "mathematical_polynomial" },
  { old: "📊️statistics/⚡️implementations/🦀️rust", dest: "📊️statistics/🦀️component.rs", mod: "statistics", lib: "mathematical_statistics" },
  { old: "📋️tabular/⚡️implementations/🦀️rust", dest: "📋️tabular/🦀️component.rs", mod: "tabular", lib: "mathematical_tabular" },
  { old: "📐️geometry/⚡️implementations/🦀️rust", dest: "📐️geometry/🦀️component.rs", mod: "geometry", lib: "mathematical_geometry" },
  { old: "📶️signal/⚡️implementations/🦀️rust", dest: "📶️signal/🦀️component.rs", mod: "signal", lib: "mathematical_signal" },
  { old: "🔗️causal/⚡️implementations/🦀️rust", dest: "🔗️causal/🦀️component.rs", mod: "causal", lib: "mathematical_causal" },
  { old: "🔢️number/⚡️implementations/🦀️rust", dest: "🔢️number/🦀️component.rs", mod: "number", lib: "mathematical_number" },
  { old: "🔷️lie/⚡️implementations/🦀️rust", dest: "🔷️lie/🦀️component.rs", mod: "lie", lib: "mathematical_lie" },
  { old: "🗺️spatial/⚡️implementations/🦀️rust", dest: "🗺️spatial/🦀️component.rs", mod: "spatial", lib: "mathematical_spatial" },
  { old: "🧮️cas/⚡️implementations/🦀️rust", dest: "🧮️cas/🦀️component.rs", mod: "cas", lib: "mathematical_cas" },
  { old: "🕸️graph/⚡️implementations/🦀️rust", dest: "🕸️graph/🦀️component.rs", mod: "graph", lib: "mathematical_graph" },
  { old: "🕸️graph/🚶️traversal/⚡️implementations/🦀️rust", dest: "🕸️graph/🚶️traversal/🦀️component.rs", mod: "graph::traversal", lib: "mathematical_graph_traversal" },
  { old: "🕸️graph/🔧️operators/⚡️implementations/🦀️rust", dest: "🕸️graph/🔧️operators/🦀️component.rs", mod: "graph::operators", lib: "mathematical_graph_operators" },
  { old: "🕸️graph/🖊️drawing/⚡️implementations/🦀️rust", dest: "🕸️graph/🖊️drawing/🦀️component.rs", mod: "graph::drawing", lib: "mathematical_graph_drawing" },
  { old: "🕸️graph/🗣️dsl/⚡️implementations/🦀️rust", dest: "🕸️graph/🗣️dsl/🦀️component.rs", mod: "graph::dsl", lib: "mathematical_graph_dsl" },
  { old: "🕸️graph/🛂️manifest/⚡️implementations/🦀️rust", dest: "🕸️graph/🛂️manifest/🦀️component.rs", mod: "graph::manifest", lib: "mathematical_graph_manifest" },
  { old: "🕸️graph/➕️normal/↔undirected/⚡️implementations/🦀️rust", dest: "🕸️graph/➕️normal/↔undirected/🦀️component.rs", mod: "graph::normal::undirected", lib: "mathematical_graph_normal_undirected" },
  { old: "🕸️graph/➕️normal/➡️directed/⚡️implementations/🦀️rust", dest: "🕸️graph/➕️normal/➡️directed/🦀️component.rs", mod: "graph::normal::directed", lib: "mathematical_graph_normal_directed" },
  { old: "🕸️graph/🔌️ports/↔undirected/⚡️implementations/🦀️rust", dest: "🕸️graph/🔌️ports/↔undirected/🦀️component.rs", mod: "graph::ports::undirected", lib: "mathematical_graph_port_undirected" },
  { old: "🕸️graph/🔌️ports/➡️directed/➕️normal/⚡️implementations/🦀️rust", dest: "🕸️graph/🔌️ports/➡️directed/➕️normal/🦀️component.rs", mod: "graph::ports::directed::normal", lib: "mathematical_graph_port_directed_normal" },
];

/** 🧩️ wfc's 40 `📂️src` modules become their own component folders (Shape V2 tree purity). */
const WFC_FOLDERS = {
  beam: "🔦️beam",
  bitset: "🎛️bitset",
  chunk: "🍰️chunk",
  constraint: "⛓️constraint",
  constraints_card: "🔢️constraints-card",
  constraints_conn: "🔗️constraints-conn",
  diag: "🩺️diag",
  domain: "🌐️domain",
  error: "⚠️error",
  evolve: "🧬️evolve",
  extract: "⛏️extract",
  flow: "🌊️flow",
  grid2d: "🔲️grid-2d",
  grid3d: "🧊️grid-3d",
  heuristics: "🧭️heuristics",
  hierarchy: "🪜️hierarchy",
  ids: "🆔️ids",
  model: "🏗️model",
  motif: "🎼️motif",
  nogood: "🚫️nogood",
  oracle: "🔮️oracle",
  outcome: "🏁️outcome",
  parallel: "🧵️parallel",
  prop_ac3: "🔁️prop-ac3",
  prop_ac4: "🔄️prop-ac4",
  propagate: "📣️propagate",
  repair: "🔧️repair",
  sample: "🎲️sample",
  search: "🔍️search",
  serial: "💾️serial",
  soft: "🪶️soft",
  solver_graph: "🕸️solver-graph",
  solver_grid2d: "🔳️solver-grid-2d",
  solver_grid3d: "🧱️solver-grid-3d",
  sparse3d: "🕳️sparse-3d",
  symmetry: "🪞️symmetry",
  tiled: "🀄️tiled",
  topology: "🗺️topology",
  trail: "🐾️trail",
  weights: "⚖️weights",
};

/** 🗑️ Empty NetworkX-parity placeholder crates deleted outright (zero real consumers, verified by grep). */
const STUBS = [
  "🕸️graph/🌊️flow",
  "🕸️graph/🌳️trees",
  "🕸️graph/🎨️coloring",
  "🕸️graph/🎯️approximation",
  "🕸️graph/🎯️centrality",
  "🕸️graph/🎯️similarity",
  "🕸️graph/🎲️generate",
  "🕸️graph/🏗️structure",
  "🕸️graph/🏘️community",
  "🕸️graph/📊️spectral",
  "🕸️graph/🔀️bipartite",
  "🕸️graph/🔄️cycles",
  "🕸️graph/🔌️io",
  "🕸️graph/🔗️connectivity",
  "🕸️graph/🔺️cliques",
  "🕸️graph/🕸️dag",
  "🕸️graph/🗺️planarity",
  "🕸️graph/🛤️paths",
  "🕸️graph/🤝️matching",
  "🕸️graph/🧩️clustering",
  "🕸️graph/🧩️components",
  "🕸️graph/🪞️isomorphism",
];

const LIB_TO_MOD = new Map(SURVIVORS.map((s) => [s.lib, s.mod]));
LIB_TO_MOD.set("mathematical_wfc", "wfc");
const LIB_NAMES = [...LIB_TO_MOD.keys()].sort((a, b) => b.length - a.length);

/** ✏️ Rewrite one moved source: own-crate `crate::` paths gain the new module prefix, sibling crate names become `crate::…`. */
function rewrite(source, ownMod) {
  let out = source;
  if (ownMod) out = out.replace(/(?<![\w])crate::/g, `crate::${ownMod}::`);
  for (const lib of LIB_NAMES) {
    out = out.replace(new RegExp(`(?<![\\w])${lib}(?![\\w])`, "g"), `crate::${LIB_TO_MOD.get(lib)}`);
  }
  return out.replace(/crate::crate::/g, "crate::");
}

function write(path, text) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
}

//#region 🚚️Move
for (const s of SURVIVORS) {
  const src = join(MATH, s.old, "📦️lib.rs");
  write(join(MATH, s.dest), rewrite(readFileSync(src, "utf8"), s.mod));
  console.log(`component ${s.dest}`);
}

const wfcLib = readFileSync(join(MATH, "🧩️wfc/⚡️implementations/🦀️rust/📦️lib.rs"), "utf8");
const wfcMods = [...wfcLib.matchAll(/#\[path = "[^"]*📂️src\/🦀️(\w+)\.rs"\]\n(pub(?:\(crate\))?) mod (\w+);/g)].map((m) => ({ file: m[1], vis: m[2], name: m[3] }));
if (wfcMods.length !== 40) throw new Error(`expected 40 wfc modules, found ${wfcMods.length}`);
for (const m of wfcMods) {
  const folder = WFC_FOLDERS[m.name];
  if (!folder) throw new Error(`no folder name for wfc module ${m.name}`);
  const src = join(MATH, "🧩️wfc/⚡️implementations/🦀️rust/📂️src", `🦀️${m.file}.rs`);
  write(join(MATH, "🧩️wfc", folder, "🦀️component.rs"), rewrite(readFileSync(src, "utf8"), "wfc"));
}
writeFileSync(
  join(REPO, ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/FRAMEWORK-MATH-FAMILY-CRATE-CONSOLIDATION/🧪️wfc-modules.json"),
  JSON.stringify(wfcMods.map((m) => ({ ...m, folder: WFC_FOLDERS[m.name] })), null, 2),
);
console.log(`wfc: ${wfcMods.length} components`);

const genSrc = join(MATH, "🕸️graph/🛂️manifest/⚡️implementations/🦀️rust/🤖️generated");
const genDest = join(MATH, "🤖️generated");
mkdirSync(genDest, { recursive: true });
for (const name of readdirSync(genSrc)) {
  const text = readFileSync(join(genSrc, name), "utf8");
  writeFileSync(join(genDest, name), name.endsWith(".rs") ? rewrite(text, "graph::manifest") : text);
}
console.log(`generated: ${readdirSync(genDest).length} files`);

write(join(MATH, "🕸️graph/🛂️manifest/🫀️core/📦️index.ts"), readFileSync(join(MATH, "🕸️graph/🛂️manifest/⚡️implementations/🦀️rust/🫀️core/📦️index.ts"), "utf8"));
//#endregion 🚚️Move

//#region 🗑️Delete
for (const s of [...SURVIVORS.map((x) => x.old), "🧩️wfc/⚡️implementations/🦀️rust"]) rmSync(join(MATH, s.replace(/\/🦀️rust$/, "")), { recursive: true, force: true });
for (const stub of [...STUBS, "🕸️graph/🎲️generate"]) rmSync(join(MATH, stub), { recursive: true, force: true });
//#endregion 🗑️Delete

const remaining = [];
(function walk(dir) {
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (name.isDirectory()) {
      if (name.name === "⚡️implementations") remaining.push(join(dir, name.name));
      else walk(join(dir, name.name));
    }
  }
})(MATH);
console.log(`remaining ⚡️implementations dirs: ${remaining.length}`);
for (const r of remaining) console.log(`  ${r}`);
