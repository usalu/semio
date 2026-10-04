/** 🔎️ Dry run: renders the schema scope catalog as `schema generate` does and names the scopes whose entry differs from the committed file, without writing anything. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const { inventorySchemaScopes, renderSchemaCatalog, loadCatalogTaxonomy } = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
const taxonomy = loadCatalogTaxonomy();
const rendered = renderSchemaCatalog(inventorySchemaScopes(ROOT, taxonomy).catalog);
const current = readFileSync(join(ROOT, taxonomy.schemaExportResolution.catalogPath), "utf8");
const scopes = (text: string): Record<string, unknown> => (JSON.parse(text) as { scopes: Record<string, unknown> }).scopes;
const was = scopes(current);
const now = scopes(rendered);
const changed = Object.keys(now).filter((scope) => JSON.stringify(was[scope]) !== JSON.stringify(now[scope]));
const gone = Object.keys(was).filter((scope) => !(scope in now));
console.log(JSON.stringify({ identical: current === rendered, changed, gone }, null, 1));
for (const scope of changed.filter((scope) => scope.includes("quiz"))) {
  const before = JSON.stringify(was[scope], null, 1).split("\n");
  const after = JSON.stringify(now[scope], null, 1).split("\n");
  const set = new Set(before);
  console.log(scope, after.filter((line) => !set.has(line)).slice(0, 20).join("\n"));
}
