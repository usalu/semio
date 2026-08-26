import { readFileSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, fixtureUrisIn, resolveFixtures } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const rows: any[] = [];
for (const c of discoverTestCases(root)) {
  if (!c.case.startsWith("mutate-")) continue;
  const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
  const uris = fixtureUrisIn(f);
  const { fixtures, missing } = resolveFixtures(root, c, uris);
  const detail = fixtures.map((x) => ({ uri: x.uri, path: x.path, bytes: existsSync(join(root, x.path)) ? statSync(join(root, x.path)).size : -1, vector: /🧬️mutations\/[^/]+\/🧪️tests\//.test(x.path) }));
  const artifacts = detail.filter((d) => !d.vector);
  const maxArtifact = artifacts.length ? Math.max(...artifacts.map((a) => a.bytes)) : 0;
  rows.push({ owner: c.owner, case: c.case, featurePath: c.featurePath, scenarios: f.scenarios.length, oracle: f.oracle, noOracle: f.noOracleDecision, vectors: detail.length - artifacts.length, artifacts, maxArtifact, missing });
}
rows.sort((a, b) => a.maxArtifact - b.maxArtifact || a.case.localeCompare(b.case));
console.log(JSON.stringify(rows, null, 1));
