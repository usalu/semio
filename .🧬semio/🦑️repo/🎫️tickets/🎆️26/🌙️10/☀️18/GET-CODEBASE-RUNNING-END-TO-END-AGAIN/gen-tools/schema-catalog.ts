import { writeFileSync } from "node:fs";
import { inventorySchemaScopes, renderSchemaCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const inventory = inventorySchemaScopes(root, loadCatalogTaxonomy());
writeFileSync(process.argv[2]!, renderSchemaCatalog(inventory.catalog));
console.log("diagnostics", inventory.diagnostics.length);
