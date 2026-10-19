// 🔬 Prints catalog problems as one line each. usage: bun check-catalogs2.ts <file>...
import {readFileSync} from "node:fs";
import {mutationCatalogProblems} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧮️mutation-catalog/🟦️.ts";
for (const f of process.argv.slice(2)) {
  const d = JSON.parse(readFileSync(f, "utf8")); const owner = f.replace(/\/🔮️oracles\/🔣️\.json$/u, "");
  for (const catalog of d.mutationCatalogs ?? []) for (const p of mutationCatalogProblems(catalog, owner)) console.log(owner.split("/🗿️artifacts/")[1] ?? owner, "|", p);
}
