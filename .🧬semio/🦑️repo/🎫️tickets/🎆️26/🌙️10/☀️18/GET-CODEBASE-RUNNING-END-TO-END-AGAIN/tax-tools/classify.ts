// 🔬 Prints the package-glue decision (role and evidence) for each given file. usage: bun classify.ts <rust|typescript|go|...> <file>...
import {readFileSync} from "node:fs";
import {classifyPackageSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const taxonomy = loadCatalogTaxonomy();
const [lang, ...files] = process.argv.slice(2);
for (const f of files) console.log(JSON.stringify(classifyPackageSource(readFileSync(f, "utf8"), (taxonomy as any).packageGlueGrammar[lang!])), f.slice(-90));
