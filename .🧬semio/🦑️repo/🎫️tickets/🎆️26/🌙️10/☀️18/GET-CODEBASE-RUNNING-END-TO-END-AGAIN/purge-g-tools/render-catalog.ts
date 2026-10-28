import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
const library = resolve(process.argv[2]!);
const { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument } = await import(resolve(library, "🔍️discovery/🧾️schema-scope-catalog/🟦️.ts"));
const { loadCatalogTaxonomy } = await import(resolve(library, "🔍️discovery/🔐️taxonomy-authority/🟦️.ts"));
const inventory = inventorySchemaScopes(process.cwd(), loadCatalogTaxonomy());
writeFileSync(process.argv[3]!, renderSchemaCatalog(inventory.catalog));
writeFileSync(process.argv[4]!, renderSchemaCatalogDocument(inventory.catalog));
console.log("rendered");
