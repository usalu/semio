import { clauses, lawFile } from "./ct.ts";
import { readSourceFamilySource, readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
const R = "/Users/ueli/Documents/semio";
const F = `${R}/🧰️framework`;
const P = `${F}/🛍️products/💻️os/🔨️modules/🔌️plugin`;
const E = `${R}/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`;
const rd = (p: string) => (existsSync(p) ? readFileSync(p, "utf8") : "");
const tree = (p: string) => (existsSync(p) ? readModuleTreeSource(p) : "");
const mode = process.argv[2] ?? "old";
const fam = (p: string) => readSourceFamilySource(p, { tests: mode === "tests" });
const args = [
  mode !== "old" ? fam(`${E}/🧵️session`) : tree(`${E}/🧵️session/🦀️.rs`),
  mode !== "old" ? fam(E) : tree(`${E}/🦀️.rs`),
  mode !== "old" ? fam(`${E}/🎭️modes/✏️edit/🪟️windows/🧱️model`) : tree(`${E}/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs`),
  mode !== "old" ? fam(`${R}/✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh`) : tree(`${R}/✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs`),
  rd(`${R}/🌎️hub/🧩️compositions/🏗️fem/🦀️.rs`),
  rd(`${R}/🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/Cargo.toml`),
  rd(`${P}/⚛️reactor/💼️jobs/🦀️.rs`),
  mode !== "old" ? fam(`${R}/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses`) : rd(`${R}/✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs`),
  fam(`${P}/⚛️reactor`),
  fam(`${F}/🛍️products/💻️os/🔨️modules/🏪️store`) + "\n" + rd(`${F}/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs`),
  fam(P),
];
console.log(args.map((a) => a.length).join(" "));
clauses(`${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🪟️fem2d-mounted-session/🟦️.ts`, "toolJobFem2dMountedSessionExact", args);
