import { readFileSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, fixtureUrisIn, resolveFixtures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const out: any[] = [];
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  const uris = fixtureUrisIn(f);
  const { fixtures, missing } = resolveFixtures(root, c, uris);
  out.push({ case: c.case, owner: c.owner, oracle: f.oracle, noOracle: f.noOracleDecision, scenarios: f.scenarios.length,
    fixtures: fixtures.map((x) => ({ uri: x.uri, path: x.path, bytes: existsSync(x.path) ? statSync(x.path).size : -1 })), missing });
}
console.log(JSON.stringify(out, null, 1));
