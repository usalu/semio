import { clauses, lawFile } from "./ct.ts";
import { readSourceFamilySource, readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
const R = "/Users/ueli/Documents/semio";
const F = `${R}/🧰️framework`;
const E = `${R}/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`;
const G = `${R}/✏️s/🔨️modules/🏗️fem/⚙️engine`;
const rd = (p: string) => (existsSync(p) ? readFileSync(p, "utf8") : "");
const tree = (p: string) => (existsSync(p) ? readModuleTreeSource(p) : "");
const fam = (p: string) => readSourceFamilySource(p, { tests: true });
const mode = process.argv[2] ?? "new";
const args = mode === "old"
  ? [tree(`${G}/🔢️sparse/🦀️.rs`), tree(`${G}/🕸️mesh/🦀️.rs`), rd(`${G}/🧮️analyses/🦀️.rs`), rd(`${G}/🏗️model/🦀️.rs`), tree(`${G}/📏️elements2d/🦀️.rs`), tree(`${E}/🧵️session/🦀️.rs`), tree(`${F}/🔨️modules/🧵️job/🦀️.rs`)]
  : [fam(`${G}/🔢️sparse`), fam(`${G}/🕸️mesh`), fam(`${G}/🧮️analyses`), fam(`${G}/🏗️model`), fam(`${G}/📏️elements2d`), fam(`${E}/🧵️session`), fam(`${F}/🔨️modules/🧵️job`)];
console.log(args.map((a) => a.length).join(" "));
clauses(`${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🧮️fem-numerical-microcursor/🟦️.ts`, "toolJobFemNumericalMicrocursorExact", args);
