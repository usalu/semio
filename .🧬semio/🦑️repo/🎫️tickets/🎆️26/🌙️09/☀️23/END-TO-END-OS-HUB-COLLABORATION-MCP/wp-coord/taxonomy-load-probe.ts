import { loadTaxonomy, loadCatalogTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const started = performance.now();
loadTaxonomy();
loadCatalogTaxonomy();
console.log(`[DEBUG] taxonomy valid in ${Math.round(performance.now() - started)} ms`);
