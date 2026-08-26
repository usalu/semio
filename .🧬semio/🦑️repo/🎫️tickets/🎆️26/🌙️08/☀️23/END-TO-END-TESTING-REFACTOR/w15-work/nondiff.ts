import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const root = "/Users/ueli/Documents/semio";
const reg = loadOracleRegistry(root);
const oracleById = new Map(reg.oracles.map((o) => [o.id, o]));
const rows: any[] = [];
let oracleScen = 0, diffScen = 0;
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  if (!f.oracle) continue;
  const byMode: Record<string, number> = {};
  for (const s of f.scenarios) byMode[s.mode] = (byMode[s.mode] ?? 0) + 1;
  oracleScen += f.scenarios.length;
  diffScen += byMode["differential"] ?? 0;
  const nonDiff = f.scenarios.length - (byMode["differential"] ?? 0);
  if (nonDiff === 0) continue;
  const e = oracleById.get(f.oracle);
  rows.push({ owner: c.ownerName, case: c.case, caseDir: c.caseDir, scenarios: f.scenarios.length, byMode, nonDiff,
    oracle: f.oracle, ecosystem: e?.ecosystem ?? null, pkg: (e as any)?.package ?? null, ver: (e as any)?.version ?? null,
    adapters: Object.keys(c.adapters),
    nonDiffScenarios: f.scenarios.filter((s) => s.mode !== "differential").map((s) => `${s.mode}:${s.id}`) });
}
rows.sort((a, b) => b.nonDiff - a.nonDiff);
console.log(JSON.stringify({ oracleScen, diffScen, nonDiff: oracleScen - diffScen, cases: rows.length, rows }, null, 1));
