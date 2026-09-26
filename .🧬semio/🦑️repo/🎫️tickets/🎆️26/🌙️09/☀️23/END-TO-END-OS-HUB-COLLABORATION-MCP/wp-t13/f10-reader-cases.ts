import { discoverTestCases, buildCasePlan, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(repoRoot);
for (const discovered of discoverTestCases(repoRoot)) {
  let plan; try { plan = buildCasePlan(repoRoot, discovered, "exhaustive" as never, registry).plan; } catch { continue; }
  if (plan.oracle === null) continue;
  const oracle = registry.oracles.find((c) => c.id === plan.oracle);
  if (oracle?.ecosystem !== "javascript") continue;
  console.log(`${oracle.id}\t${oracle.kind}\tinput=${plan.oracleInput ?? "-"}\t${Object.keys(discovered.adapters).join(",")}\t${discovered.caseDir}`);
}
