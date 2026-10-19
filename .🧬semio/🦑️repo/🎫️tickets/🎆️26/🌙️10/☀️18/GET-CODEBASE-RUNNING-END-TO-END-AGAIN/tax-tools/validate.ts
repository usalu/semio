// 🔬 Runs validateTaxonomy over the live taxonomy document and prints the problem count and messages. usage: bun validate.ts
import {loadCatalogTaxonomy, validateTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const problems = validateTaxonomy(loadCatalogTaxonomy() as any);
console.log(JSON.stringify({problems: problems.length}));
for (const p of problems) console.log(p);
