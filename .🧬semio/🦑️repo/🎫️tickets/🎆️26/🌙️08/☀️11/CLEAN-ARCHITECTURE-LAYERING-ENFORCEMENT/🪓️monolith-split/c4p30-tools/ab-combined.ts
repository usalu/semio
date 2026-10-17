// 🅰️ c4p30: A/B on a gate sandbox. Copies the combined sandbox of an applied-layout run into <copy>, restores the unsplit snapshot of each given root (rewritten for the combined layout exactly like `mountCrates`) and deletes the split child folders. usage: bun ab-combined.ts <combined sandbox> <copy dir> <root relative path>=<orig snapshot name>=<crate ident>=<child folder,child folder>...
import { cpSync, readFileSync, rmSync, writeFileSync, realpathSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { rewriteCrateAliases, rewriteForCombined } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { patchSandboxSources } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
const [combined, copy, ...rows] = process.argv.slice(2) as string[];
const ORIG = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p30-tools/orig";
rmSync(copy, { recursive: true, force: true });
mkdirSync(copy, { recursive: true });
cpSync(combined, copy, { recursive: true });
const under = (path: string): void => { const real = join(realpathSync(dirname(path)), "x"); if (!real.startsWith(realpathSync(copy) + "/")) throw new Error("write outside the copy: " + path); };
const crateModules = new Map<string, string>();
for (const m of readFileSync(join(copy, "__combined.rs"), "utf8").matchAll(/pub mod (__c_([A-Za-z0-9_]+));/gu)) crateModules.set(m[2]!, m[1]!);
for (const row of rows) {
  const [root, snapshot, ident, kids] = row.split("=") as [string, string, string, string];
  const module = crateModules.get(ident);
  if (!module) throw new Error("unknown crate ident " + ident);
  const target = join(copy, root);
  under(target);
  const text = rewriteCrateAliases(rewriteForCombined(readFileSync(join(ORIG, snapshot), "utf8"), module, crateModules), module, crateModules);
  writeFileSync(target, text);
  for (const kid of kids.split(",")) { const dir = join(dirname(target), kid); under(join(dir, "x")); rmSync(dir, { recursive: true, force: true }); }
}
patchSandboxSources(copy, ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"]);
console.log("copy ready", copy, "roots", rows.length);
