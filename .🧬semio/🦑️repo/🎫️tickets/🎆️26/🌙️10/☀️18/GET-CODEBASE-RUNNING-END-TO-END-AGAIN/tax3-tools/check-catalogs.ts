// 🔬 Validates the mutation catalogs of changed oracle files with the test framework's own catalog validator. usage: bun check-catalogs.ts <file>...
import {readFileSync} from "node:fs";
import {mutationCatalogProblems} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧮️mutation-catalog/🟦️.ts";
let bad = 0;
for (const f of process.argv.slice(2)) {
  const d = JSON.parse(readFileSync(f, "utf8"));
  const owner = f.replace(/\/🔮️oracles\/🔣️\.json$/u, "");
  for (const catalog of d.mutationCatalogs ?? []) { const problems = mutationCatalogProblems(catalog, owner); if (problems.length) { bad += problems.length; console.log(f.slice(-90), problems.slice(0, 3)); } }
}
console.log("problems", bad);
