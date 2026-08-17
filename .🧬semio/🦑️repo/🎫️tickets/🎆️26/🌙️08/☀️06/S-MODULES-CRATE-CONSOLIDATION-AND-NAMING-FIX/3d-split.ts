#!/usr/bin/env bun
/**
 * 🧊️ One-shot splitter for the `semio-s-3d` merge (ticket
 * `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX`).
 *
 * Carves the five old `✏️s/🔨️modules/🧊️3d/**` crate `📦️lib.rs` files into the Shape-V2 taxonomy
 * tree at the `🧊️3d/` owner root. Purely mechanical: bodies are copied verbatim, de-indented by
 * exactly four columns where they were nested inside a `pub mod … { … }` block, and the only text
 * rewrites are the crate-path requalifications listed in `REQUALIFY` (old sibling-crate / old
 * crate-root module paths → their new `crate::brep::…` homes).
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const OWNER = join(REPO, "✏️s/🔨️modules/🧊️3d");
const LEAF = "🦀️component.rs";

const oldLib = (p: string) => join(OWNER, p, "⚡️implementations/🦀️rust/📦️lib.rs");

//#region 🔖️Rewrites
/** 🔁️ Old absolute path → new absolute path. Applied to every emitted component file. */
const BREP_MODULES = ["curve_ops", "surface_ops", "error", "vec", "mat", "tolerance", "predicates", "oracle", "poly", "bezier", "bspline", "curve", "surface", "arena", "history", "topo", "euler", "validate"];

function requalify(source: string): string {
  let out = source;
  for (const m of BREP_MODULES) out = out.replace(new RegExp(`crate::${m}\\b`, "g"), `crate::brep::${m}`);
  out = out.replace(/\bkernel_3d_engine::/g, "crate::brep::engine::");
  out = out.replace(/\[`kernel_3d_engine`\]/g, "[`crate::brep::engine`]");
  out = out.replace(/`kernel_3d_brepkit`/g, "`crate::brep::kernel`");
  return out;
}
//#endregion 🔖️Rewrites

//#region 🔖️Emit
function emit(relDir: string, body: string): void {
  const target = join(OWNER, relDir, LEAF);
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, requalify(body).replace(/\n*$/, "\n"), "utf8");
  console.log(`${relDir}/${LEAF}  ${body.split("\n").length} lines`);
}

/** ✂️ Extract `[from, to]` (1-based, inclusive) and drop exactly one indent level. */
function dedent(lines: string[], from: number, to: number): string {
  return lines
    .slice(from - 1, to)
    .map((l) => (l.startsWith("    ") ? l.slice(4) : l))
    .join("\n");
}
//#endregion 🔖️Emit

//#region 🔖️Brep
const brep = readFileSync(oldLib("📐️brep"), "utf8").split("\n");

/** 📐️ `[folder, rust module name, `pub mod X {` line, `// #endregion` line]` (1-based). */
const NATIVE: Array<[string, string, number, number]> = [
  ["🚨️error", "error", 8, 193],
  ["➡️vector", "vec", 196, 596],
  ["🔢️matrix", "mat", 599, 949],
  ["📏️tolerance", "tolerance", 952, 1189],
  ["⚖️predicates", "predicates", 1192, 1538],
  ["🔮️oracle", "oracle", 1541, 1684],
  ["〰️polynomial", "poly", 1687, 2158],
  ["🎢️bezier", "bezier", 2161, 2488],
  ["🪢️bspline", "bspline", 2491, 2907],
  ["➰️curve", "curve", 2910, 3312],
  ["✂️curve-ops", "curve_ops", 3315, 3781],
  ["🏄️surface", "surface", 3784, 4100],
  ["🪡️surface-ops", "surface_ops", 4103, 4319],
  ["🏟️arena", "arena", 4322, 4584],
  ["📜️history", "history", 4587, 4747],
  ["🕸️topology", "topo", 4750, 5104],
  ["🔺️euler", "euler", 5107, 5386],
  ["✅️validate", "validate", 5389, 5634],
];
for (const [folder, name, open, end] of NATIVE) {
  if (brep[open - 1] !== `pub mod ${name} {`) throw new Error(`${name}: line ${open} is ${JSON.stringify(brep[open - 1])}`);
  if (brep[end - 2] !== "}") throw new Error(`${name}: line ${end - 1} is ${JSON.stringify(brep[end - 2])}, expected the module's closing brace`);
  emit(join("📐️brep", folder), dedent(brep, open + 1, end - 2));
}

const KERNEL_DOC = [
  "//! 🔩️ Brepkit-backed implementation of [`crate::brep::engine::BrepKernel`] (being replaced in place by",
  "//! a dependency-free native kernel — see `.🦑️repo/🎫️tickets/26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`).",
  "//! The native modules alongside it are additive: they compile alongside the brepkit wrapper until the",
  "//! ticket's Flip phase swaps consumers over and deletes the wrapper.",
  "",
].join("\n");
emit(join("📐️brep", "🧰️kernel"), KERNEL_DOC + brep.slice(5637, 8436).join("\n"));
//#endregion 🔖️Brep

//#region 🔖️Engine
const engine = readFileSync(oldLib("📐️brep/⚙️engine"), "utf8").split("\n");
if (engine[2] !== "pub mod compute {" || engine[16] !== "}" || engine[18] !== "pub use compute::block_on;") throw new Error("engine layout drifted");
emit(join("📐️brep", "⚙️engine", "🧮️compute"), dedent(engine, 4, 16));
emit(join("📐️brep", "⚙️engine"), [engine[0], "", "pub use crate::brep::engine::compute::block_on;", ...engine.slice(19)].join("\n"));
//#endregion 🔖️Engine

//#region 🔖️FlatCrates
emit("🥽️mesh", readFileSync(oldLib("🥽️mesh"), "utf8"));
emit("🎬️scene", readFileSync(oldLib("🎬️scene"), "utf8"));
emit("🗺️spatial", readFileSync(oldLib("🗺️spatial"), "utf8"));
//#endregion 🔖️FlatCrates
