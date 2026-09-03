import { findRepoRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { loadOracleRegistry, reimplementationOracleBreaches, isQualifyingOracleKind } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = findRepoRoot("/Users/ueli/Documents/semio");
const registry = loadOracleRegistry(repoRoot);
const breaches = reimplementationOracleBreaches(repoRoot, registry);
console.log("TOTAL BREACHES:", breaches.length);
for (const b of breaches) {
  const owner = b.scope.replace(/\/🦀️oracle\.rs$/, "");
  const contribution = registry.contributions.find((c: any) => c.owner === owner);
  console.log("===", owner, "===");
  if (!contribution) { console.log("  NO CONTRIBUTION FOUND"); continue; }
  const qualifying = contribution.oracles.filter((o: any) => isQualifyingOracleKind(o.kind));
  for (const o of qualifying) {
    console.log(`  id=${o.id} kind=${o.kind} ecosystem=${o.ecosystem} hostPath=${o.hostPath ?? ""}`);
  }
}
