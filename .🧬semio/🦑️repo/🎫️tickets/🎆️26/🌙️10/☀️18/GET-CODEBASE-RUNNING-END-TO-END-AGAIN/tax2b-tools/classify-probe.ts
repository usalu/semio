import {readFileSync} from "node:fs";
import {classifyPackageSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
for (const p of process.argv.slice(2)) console.log(JSON.stringify(classifyPackageSource(readFileSync(p, "utf8"), grammar)), p.slice(-70));
