// Hand-off, NOT run by slice rtv (📚️library is held by the repo-lib work). Run once the hold is lifted: `bun apply-library-repoints.mjs --write`.
import { readFileSync, writeFileSync } from "node:fs";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const edits = [
  [L + "🧪️tests/🧭️fixture-placement/🟦️.ts", 'import { inspectTestLayoutSources, testTaxonomy } from "../../../🧪️test/🟦️.ts";', 'import { inspectTestLayoutSources } from "../../../🧪️test/🗺️test-layout/🟦️.ts";\nimport { testTaxonomy } from "../../../🧪️test/🗂️taxonomy/🟦️.ts";'],
  [L + "🧪️tests/🔏️path-emoji-statutes/🟦️.ts", 'readFileSync(resolve(root, "../../../🧪️test/🟦️.ts"), "utf8")', 'readFileSync(resolve(root, "../../../🧪️test/📃️case-contract/🟦️.ts"), "utf8")'],
  [L + "📦️packages/🟦️typescript/📋️project.json", '        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts"\n', '        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🗂️taxonomy/🟦️.ts",\n        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🗺️test-layout/🟦️.ts",\n        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📃️case-contract/🟦️.ts"\n', 2],
];
for (const [file, from, to, count = 1] of edits) {
  const text = readFileSync(file, "utf8");
  const found = text.split(from).length - 1;
  if (found !== count) throw new Error(`${file}: expected ${count} occurrence(s) of the anchor, found ${found}`);
  console.log(`${file}: ${found} edit(s)`);
  if (process.argv.includes("--write")) writeFileSync(file, text.split(from).join(to));
}
