import { writeFileSync } from "node:fs";
import { inventorySchemaScopes, renderSchemaCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2]!;
const inventory = inventorySchemaScopes(root);
writeFileSync(out, renderSchemaCatalog(inventory.catalog));
console.log(`scopes=${Object.keys(inventory.catalog.scopes).length} diagnostics=${inventory.diagnostics.length}`);
