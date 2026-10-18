import { writeFileSync } from "node:fs";
import { inventorySchemaScopes, renderSchemaCatalog, loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const inventory = inventorySchemaScopes(root, loadCatalogTaxonomy());
writeFileSync(process.argv[2]!, renderSchemaCatalog(inventory.catalog));
console.log("diagnostics", inventory.diagnostics.length);
