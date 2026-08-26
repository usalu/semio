import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const root = "/Users/ueli/Documents/semio";
const cases = discoverTestCases(root);
const reg = loadOracleRegistry(root);
const oracleById = new Map(reg.oracles.map((o) => [o.id, o]));
const noOracleById = new Map(reg.noOracleDecisions.map((o) => [o.id, o]));

let totalScenarios = 0;
const rows: any[] = [];
for (const c of cases) {
  const src = readFileSync(join(root, c.featurePath), "utf8");
  const f = parseFeature(src);
  totalScenarios += f.scenarios.length;
  const e = f.oracle ? oracleById.get(f.oracle) : null;
  const d = f.noOracleDecision ? noOracleById.get(f.noOracleDecision) : null;
  rows.push({
    owner: c.owner, ownerName: c.ownerName, case: c.case, caseDir: c.caseDir,
    scenarios: f.scenarios.length,
    modes: [...new Set(f.scenarios.map((s) => s.mode))],
    levels: [...new Set(f.scenarios.map((s) => s.level))],
    capability: f.capability, oracle: f.oracle, noOracle: f.noOracleDecision,
    comparison: f.comparison, catalog: f.mutationCatalog,
    oracleEcosystem: e ? e.ecosystem : null, oraclePackage: e ? e.package : null, oracleVersion: e ? (e as any).version ?? null : null,
    oracleRationale: e ? (e as any).rationale ?? null : null,
    noOracleRationale: d ? d.rationale : null, noOracleSubstitutes: d ? d.substitutes : null,
    adapters: Object.keys(c.adapters), errors: f.errors,
    fixtures: [...new Set(f.scenarios.flatMap((s) => s.steps.map((t) => t.text).filter((t) => /(asset|shared|local):\/\//.test(t))))].slice(0, 8),
  });
}
console.log(JSON.stringify({ cases: cases.length, totalScenarios, oracles: reg.oracles.length, noOracleDecisions: reg.noOracleDecisions.length, rows }, null, 1));
