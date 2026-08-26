import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.noOracleDecision) continue;
  const s = f.scenarios.find((x) => x.id === "identity-round-trip");
  if (!s) { console.log(`### ${c.case} :: NO identity-round-trip`); continue; }
  console.log(`### ${c.case} [${c.ownerName}] mode=${s.mode}`);
  for (const st of s.steps) console.log(`    ${st.keyword} ${st.text}`);
}
