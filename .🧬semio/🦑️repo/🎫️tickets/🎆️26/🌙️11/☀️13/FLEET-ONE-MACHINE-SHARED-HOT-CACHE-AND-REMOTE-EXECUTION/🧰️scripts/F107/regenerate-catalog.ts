import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const scratch = process.argv[2]!;
const resolution = loadCatalogTaxonomy().schemaExportResolution;
const inventory = inventorySchemaScopes(root);
mkdirSync(scratch, { recursive: true });
const outputs = [["catalog", resolution.catalogPath, renderSchemaCatalog(inventory.catalog)], ["document", resolution.catalogDocumentPath, renderSchemaCatalogDocument(inventory.catalog)]] as const;
for (const [name, rel, text] of outputs) {
  let current = "";
  try { current = readFileSync(join(root, rel), "utf8"); } catch {}
  writeFileSync(join(scratch, name), text);
  console.log(`${name} ${rel}: ${current === text ? "current" : "stale"} (${text.length} bytes)`);
}
console.log(`scopes=${Object.keys(inventory.catalog.scopes).length} definitionModules=${inventory.definitionModules.length}`);
