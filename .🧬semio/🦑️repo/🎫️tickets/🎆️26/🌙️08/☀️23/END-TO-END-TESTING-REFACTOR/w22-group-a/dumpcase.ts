import { readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverTestCases, parseFeature, buildCasePlan } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const root = "/Users/ueli/Documents/semio";
const want = process.argv[2]!;
const c = discoverTestCases(root).find((x) => x.case === want)!;
const f = parseFeature(readFileSync(join(root, c.featurePath), "utf8"));
const { plan, missingFixtures } = buildCasePlan(root, c, "exhaustive");
console.log(JSON.stringify({ case: c.case, owner: c.owner, caseDir: c.caseDir, featurePath: c.featurePath, tags: f.tags, oracle: f.oracle, noOracle: f.noOracleDecision, missingFixtures, fixtures: plan.fixtures, scenarios: f.scenarios }, null, 1));
