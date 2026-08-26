import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const cases = discoverTestCases(root);
const reg = loadOracleRegistry(root);
let scen = 0, oCase = 0, oScen = 0, nCase = 0, nScen = 0, diff = 0;
const modes: Record<string, number> = {};
const noOracle: string[] = [];
for (const c of cases) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  scen += f.scenarios.length;
  for (const s of f.scenarios) modes[s.mode] = (modes[s.mode] ?? 0) + 1;
  if (f.oracle) { oCase++; oScen += f.scenarios.length; diff += f.scenarios.filter((s) => s.mode === "differential").length; }
  if (f.noOracleDecision) { nCase++; nScen += f.scenarios.length; noOracle.push(`${f.scenarios.length}\t${c.case}\t${f.noOracleDecision}`); }
}
const claimedO = new Set(cases.map((c) => parseFeature(readFileSync(join(root, c.featurePath), "utf8")).oracle).filter(Boolean));
const claimedN = new Set(cases.map((c) => parseFeature(readFileSync(join(root, c.featurePath), "utf8")).noOracleDecision).filter(Boolean));
console.log(JSON.stringify({
  cases: cases.length, scenarios: scen,
  oracleCases: oCase, oracleScenarios: oScen, differentialScenarios: diff,
  noOracleCases: nCase, noOracleScenarios: nScen,
  registryOracles: reg.oracles.length, registryDecisions: reg.noOracleDecisions.length,
  orphanOracles: reg.oracles.map((o) => o.id).filter((id) => !claimedO.has(id)),
  orphanDecisions: reg.noOracleDecisions.map((o) => o.id).filter((id) => !claimedN.has(id)),
  modes,
}, null, 1));
console.log(noOracle.sort((a, b) => Number(b.split("\t")[0]) - Number(a.split("\t")[0])).join("\n"));
