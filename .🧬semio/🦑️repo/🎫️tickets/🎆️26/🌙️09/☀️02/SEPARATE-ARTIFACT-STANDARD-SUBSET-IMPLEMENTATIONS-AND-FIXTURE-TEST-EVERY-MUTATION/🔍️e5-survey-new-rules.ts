import { discoverTestCases, loadOracleRegistry, parseFeature, caseAboveSubsetBreaches, mutationFixtureBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(repoRoot);
const cases = discoverTestCases(repoRoot);

const ruleA = cases.flatMap((discovered) => {
  const feature = parseFeature(readFileSync(join(repoRoot, discovered.featurePath), "utf8"));
  return caseAboveSubsetBreaches(discovered, feature, registry);
});
console.log(`case-above-subset: ${ruleA.length}`);
for (const b of ruleA) console.log("  ", b.scope, "|", b.summary);

const ruleB = mutationFixtureBreaches(registry);
console.log(`mutation-without-fixture: ${ruleB.length}`);
const byArtifact = new Map<string, number>();
for (const b of ruleB) {
  const m = /Mutation .* of ([^ ]+)@/.exec(b.summary);
  const artifact = m ? m[1] : "?";
  byArtifact.set(artifact, (byArtifact.get(artifact) ?? 0) + 1);
}
for (const [a, n] of [...byArtifact.entries()].sort((x, y) => y[1] - x[1])) console.log("  ", a, n);
