import { loadOracleRegistry, mutationFixtureBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(repoRoot);
const ruleB = mutationFixtureBreaches(registry);
for (const b of ruleB) console.log(b.scope, "|", b.summary);
