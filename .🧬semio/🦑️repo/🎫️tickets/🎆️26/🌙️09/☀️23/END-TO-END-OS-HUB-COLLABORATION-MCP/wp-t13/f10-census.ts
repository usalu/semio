import { ADAPTER_ENTRY_POINTS, discoverTestCases, buildCasePlan, loadOracleRegistry, oracleImplementation, type Implementation } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(repoRoot);
const cases = discoverTestCases(repoRoot);
const rows: string[] = [];
let missingEntry = 0, missingOracleAdapter = 0;
for (const discovered of cases) {
  for (const [implementation, path] of Object.entries(discovered.adapters) as [Implementation, string][]) {
    if (!ADAPTER_ENTRY_POINTS[implementation].pattern.test(readFileSync(join(repoRoot, path), "utf8"))) { missingEntry++; rows.push(`no-entry\t${implementation}\t${discovered.caseDir}`); }
  }
  let plan;
  try { plan = buildCasePlan(repoRoot, discovered, "exhaustive" as never, registry).plan; } catch (error) { rows.push(`plan-error\t-\t${discovered.caseDir}\t${String(error).slice(0, 200)}`); continue; }
  if (plan.oracle === null) continue;
  const oracle = registry.oracles.find((candidate) => candidate.id === plan.oracle);
  if (oracle === undefined) continue;
  const mapped = oracleImplementation(oracle);
  if (mapped !== undefined && discovered.adapters[mapped] === undefined) {
    missingOracleAdapter++;
    rows.push(`no-oracle-adapter\t${mapped}\t${discovered.caseDir}\toracle=${oracle.id}`);
  }
}
console.log(rows.sort().join("\n"));
console.log(`cases=${cases.length} no-entry=${missingEntry} no-oracle-adapter=${missingOracleAdapter}`);
