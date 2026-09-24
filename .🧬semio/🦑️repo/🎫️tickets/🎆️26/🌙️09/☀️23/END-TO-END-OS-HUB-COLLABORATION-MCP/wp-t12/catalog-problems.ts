/** 🩺️ Prints `mutationCatalogProblems` for every catalog of one oracle contribution. Usage: bun catalog-problems.ts <owner> */
import { readFileSync } from "node:fs";
import { mutationCatalogProblems } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const owner = process.argv[2]!;
const manifest = JSON.parse(readFileSync(`/Users/ueli/Documents/semio/${owner}/🔮️oracles/🔣️.json`, "utf8"));
for (const catalog of manifest.mutationCatalogs) console.log(catalog.id, JSON.stringify(mutationCatalogProblems(catalog, owner)));
