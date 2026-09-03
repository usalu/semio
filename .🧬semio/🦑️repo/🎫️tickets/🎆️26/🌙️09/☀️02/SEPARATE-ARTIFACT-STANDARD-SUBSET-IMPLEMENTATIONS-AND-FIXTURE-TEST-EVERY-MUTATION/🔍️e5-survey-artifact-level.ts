import { discoverTestCases } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const cases = discoverTestCases(repoRoot);
const artifactLevel = cases.filter((c) => c.owner.includes("🗿️artifacts/") && !c.owner.includes("/🪆️subsets/"));
console.log(`total cases: ${cases.length}`);
console.log(`artifact-level (not under subsets): ${artifactLevel.length}`);
for (const c of artifactLevel) console.log(c.owner, "|", c.case);
