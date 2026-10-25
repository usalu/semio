import { readFileSync } from "node:fs";
import { classifyPackageSource, classifyPackageSourceDisposition } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadCatalogTaxonomy();
const d = t.packageSourceDispositions["root-script"]!;
const g = t.packageGlueGrammar[d.grammarId ?? "typescript"]!;
for (const file of process.argv.slice(2)) {
  const src = readFileSync(file, "utf8");
  console.log(classifyPackageSourceDisposition(src, d, g), JSON.stringify(classifyPackageSource(src, g)).slice(0, 400), file.slice(-60));
}
