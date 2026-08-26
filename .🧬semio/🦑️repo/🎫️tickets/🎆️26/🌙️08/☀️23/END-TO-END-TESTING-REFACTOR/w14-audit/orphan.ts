import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const used = new Set<string>(), usedO = new Set<string>();
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (f.noOracleDecision) used.add(f.noOracleDecision);
  if (f.oracle) usedO.add(f.oracle);
}
const reg = loadOracleRegistry(root);
console.log("decisions", reg.noOracleDecisions.length, "used", used.size);
console.log("ORPHAN decisions:", reg.noOracleDecisions.filter((d) => !used.has(d.id)).map((d) => d.id));
console.log("oracle entries", reg.oracles.length, "used", usedO.size);
console.log("ORPHAN oracle entries:", reg.oracles.filter((o) => !usedO.has(o.id)).map((o) => o.id));
