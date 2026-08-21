// 🧹️ Scoped re-run of puzzle's `fixtures lint` rules over the three 🧱️block trees only — the shared
// linter caps its printed error list at 40 repo-wide, so this proves the block rows are error-free.
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const ROOTS = ["🖐️5d", "🧊️3d", "◻2d"].map((a) => join(REPO, `✏️s/🔌️plugins/🧱️block/🗿️artifacts/${a}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`));
const NON_MUTATION_DIRS = new Set(["💾️binary", "📝️text"]);
const CORE_CASE_FILES = ["🦠️mutation/🔣️component.json", "🔺️diff/🔣️component.json", "🎯️outcome/🔣️component.json", "🦀️component.rs"];
const DERIVED_CASE_FILES = ["🦠️mutation/🔧️component.op.semio", "🦠️mutation/📡️component.spr.semio", "🔺️diff/🩹️component.patch.semio", "🔺️diff/📡️component.patch.spr.semio"];
const SNAPSHOT_DERIVED = ["🗣️component.dsl.semio", "🎒️component.pack.semio"];
const dirsIn = (p: string) => (existsSync(p) ? readdirSync(p).filter((e) => statSync(join(p, e)).isDirectory()) : []);

let errors = 0;
let warnings = 0;
for (const root of ROOTS) {
  const aggregate = readFileSync(join(root, "🦀️component.rs"), "utf8");
  const body = aggregate.match(/pub enum \w*Mutation\w* \{([\s\S]*?)\n\}/);
  const variants = body ? [...body[1].matchAll(/^\s+([A-Z][A-Za-z0-9]*)\(/gm)].map((m) => m[1]) : [];
  const leaves = dirsIn(root).filter((e) => !NON_MUTATION_DIRS.has(e)).filter((e) => existsSync(join(root, e, "🦠️mutation/🦀️component.rs")));
  const structs = new Set(leaves.map((leaf) => readFileSync(join(root, leaf, "🦠️mutation/🦀️component.rs"), "utf8").match(/^pub struct ([A-Za-z0-9]+)/m)?.[1]));
  for (const variant of variants) if (!structs.has(variant)) { console.log(`❌️ ${root}:${variant} has no mutation directory`); errors += 1; }
  let covered = 0;
  for (const leaf of leaves) {
    const cases = dirsIn(join(root, leaf, "🧪️tests"));
    if (cases.length === 0) { console.log(`❌️ ${leaf}: no 🧪️tests cases`); errors += 1; continue; }
    covered += 1;
    for (const testCase of cases) {
      const dir = join(root, leaf, "🧪️tests", testCase);
      const outcome = JSON.parse(readFileSync(join(dir, "🎯️outcome/🔣️component.json"), "utf8"));
      const rejected = outcome.status === "rejected";
      if (!["applied", "rejected"].includes(outcome.status)) { console.log(`❌️ ${leaf}/${testCase}: bad status`); errors += 1; }
      if (rejected && typeof outcome.code !== "string") { console.log(`❌️ ${leaf}/${testCase}: rejected without code`); errors += 1; }
      for (const rel of CORE_CASE_FILES) {
        if (rejected && rel.startsWith("🔺️diff/")) continue;
        if (!existsSync(join(dir, rel))) { console.log(`❌️ ${leaf}/${testCase}: missing ${rel}`); errors += 1; }
      }
      for (const rel of DERIVED_CASE_FILES) if (!existsSync(join(dir, rel))) warnings += 1;
      for (const side of ["⬅️before", "➡️after"]) {
        const sideDir = join(dir, "📸️snapshot", side);
        if (!existsSync(join(sideDir, "🔣️component.json"))) { console.log(`❌️ ${leaf}/${testCase}: missing 📸️snapshot/${side}/🔣️component.json`); errors += 1; }
        for (const name of SNAPSHOT_DERIVED) if (!existsSync(join(sideDir, name))) warnings += 1;
      }
    }
  }
  console.log(`${covered}/${leaves.length} covered · ${root.slice(REPO.length + 1)}`);
}
console.log(errors === 0 ? `✅️ block trees: 0 errors · ${warnings} derived-encoding warnings (expected)` : `❌️ ${errors} error(s)`);
