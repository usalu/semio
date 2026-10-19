// 🔬 Finds which top-level statements keep a Rust package entry from classifying as glue by classifying each statement alone. usage: bun bisect-rust.ts <file>
import {readFileSync} from "node:fs";
import {classifyPackageSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.rust;
const lines = readFileSync(process.argv[2]!, "utf8").split("\n");
for (const [i, line] of lines.entries()) {
  if (!line.trim()) continue;
  const d = classifyPackageSource(line + "\n", grammar);
  if (!["declaration", "registration", "bootstrap", "thin-delegation"].includes(d.role)) console.log(i + 1, d.role, d.evidence, "|", line.slice(0, 140));
}
