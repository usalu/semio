import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const tally: Record<string, Record<string, number>> = { oracle: {}, noOracle: {} };
const perCase: Record<string, number> = {};
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  const bucket = f.oracle ? "oracle" : "noOracle";
  for (const s of f.scenarios) { tally[bucket][s.mode] = (tally[bucket][s.mode] ?? 0) + 1; if (bucket === "oracle" && s.mode === "differential") perCase[c.case] = (perCase[c.case] ?? 0) + 1; }
}
console.log(JSON.stringify(tally, null, 1));
const ruststep = Object.entries(perCase).filter(([k]) => k.includes("ifc") || k.includes("step"));
console.log("differential scenarios in ifc/step cases:", ruststep);
