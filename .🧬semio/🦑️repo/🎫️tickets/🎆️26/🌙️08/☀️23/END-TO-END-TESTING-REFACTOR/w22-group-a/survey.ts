import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const root = "/Users/ueli/Documents/semio";
const cases = discoverTestCases(root);
const reg = loadOracleRegistry(root);
const noOracleById = new Map(reg.noOracleDecisions.map((o) => [o.id, o]));

const rows: any[] = [];
for (const c of cases) {
  const src = readFileSync(join(root, c.featurePath), "utf8");
  const f = parseFeature(src);
  if (!f.noOracleDecision) continue;
  const d = noOracleById.get(f.noOracleDecision);
  rows.push({
    owner: c.owner, ownerName: c.ownerName, case: c.case, caseDir: c.caseDir, featurePath: c.featurePath,
    scenarios: f.scenarios.length,
    scenarioIds: f.scenarios.map((s) => ({ id: s.id, mode: s.mode, level: s.level })),
    capability: f.capability, noOracle: f.noOracleDecision, comparison: f.comparison, catalog: f.mutationCatalog,
    adapters: Object.keys(c.adapters),
    rationale: d ? d.rationale : null,
    fixtures: [...new Set(f.scenarios.flatMap((s) => s.steps.flatMap((t) => (t.text.match(/(?:asset|shared|local):\/\/\S+/g) ?? []))))],
  });
}
rows.sort((a, b) => b.scenarios - a.scenarios);
console.log(JSON.stringify({ count: rows.length, rows }, null, 1));
