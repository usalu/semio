// 🔬️ Fast taxonomy gate: runs the same two validators the dev server and Storybook run at startup,
// so a fix can be checked in seconds instead of a multi-minute boot.
import { loadCatalogTaxonomy, loadTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

for (const [name, run] of [["loadCatalogTaxonomy", loadCatalogTaxonomy], ["loadTaxonomy", loadTaxonomy]] as const) {
  try {
    run();
    console.log(`✅ ${name}`);
  } catch (error) {
    console.log(`❌ ${name}: ${error instanceof Error ? error.message : String(error)}`);
    break;
  }
}
