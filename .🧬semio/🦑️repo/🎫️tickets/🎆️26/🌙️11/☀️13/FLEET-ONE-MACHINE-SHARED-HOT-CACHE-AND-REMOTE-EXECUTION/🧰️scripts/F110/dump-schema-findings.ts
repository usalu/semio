/** 🔎 Read-only dump of every `schema check` finding as JSONL (code, path, message). Same finding set as `schema check`: inventory diagnostics + placement + catalog freshness. */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { inventorySchemaScopes, renderSchemaCatalog, type SchemaScopeDiagnostic } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";

const root = resolve("/Users/ueli/Documents/semio");
const out = process.argv[2]!;
const taxonomy = loadCatalogTaxonomy();
const inventory = inventorySchemaScopes(root, taxonomy);
const rel = taxonomy.schemaExportResolution.catalogPath;
const abs = join(root, rel);
const findings: SchemaScopeDiagnostic[] = [...inventory.diagnostics, ...inventory.placement];
const rendered = renderSchemaCatalog(inventory.catalog);
if (!existsSync(abs)) findings.push({ code: "schema-catalog-missing", path: rel, detail: "catalog file absent" });
else {
  const tracked = readFileSync(abs, "utf8");
  if (tracked !== rendered) findings.push({ code: "schema-catalog-stale", path: rel, detail: "The catalog does not match the schema modules on disk." });
}
const lines = findings.map(f => JSON.stringify({ code: f.code, path: f.path, message: f.detail }));
writeFileSync(out, lines.join("\n") + "\n");
console.log(`modules=${inventory.modules.length} definitionModules=${inventory.definitionModules.length} findings=${findings.length}`);
