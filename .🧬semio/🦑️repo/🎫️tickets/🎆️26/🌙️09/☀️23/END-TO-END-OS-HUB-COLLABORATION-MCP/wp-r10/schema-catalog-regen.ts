#!/usr/bin/env bun
/**
 * 📇️ R10 session 15 (window 5, after the taxonomy step and L1's round 4): regenerates the derived schema scope catalog
 * (`📚️library/🔣️schema-catalog.json`) and its Markdown index (`📓️schema-catalog.md`) exactly as `bun ./📜️script.ts schema
 * generate` + `schema docs` render them (same discovery functions), against the live taxonomy or a candidate
 * (`--taxonomy <candidate.json>`, dry run only). A dry run writes the renders as previews into the R10 state directory and
 * prints the changed-line counts and the scopes added/removed/changed; `--apply` keeps the previous bytes as a backup
 * and writes both files. Proof after apply: `bun ./📜️script.ts schema verify` + `schema check`.
 * Usage: bun schema-catalog-regen.ts [--taxonomy <candidate.json>] [--apply]
 */
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const STATE = join(ROOT, ".🧬semio/🌐hub/s14-r10-state");
const { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument, loadCatalogTaxonomy } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`);
const args = process.argv.slice(2);
const apply = args.includes("--apply");
const candidateAt = args.indexOf("--taxonomy");
if (apply && candidateAt >= 0) throw new Error("--apply renders against the live taxonomy only");
const taxonomy = candidateAt >= 0 ? JSON.parse(readFileSync(args[candidateAt + 1]!, "utf8")) : loadCatalogTaxonomy();
const started = performance.now();
const inventory = inventorySchemaScopes(ROOT, taxonomy);
const outputs: [string, string][] = [[taxonomy.schemaExportResolution.catalogPath, renderSchemaCatalog(inventory.catalog)], [taxonomy.schemaExportResolution.catalogDocumentPath, renderSchemaCatalogDocument(inventory.catalog)]];
const backup = join(STATE, `schema-catalog-backup-${new Date().toISOString().replaceAll(":", "-")}`);
const scopesOf = (text: string): Record<string, unknown> => { try { return (JSON.parse(text) as { scopes?: Record<string, unknown> }).scopes ?? {}; } catch { return {}; } };
for (const [relative, rendered] of outputs) {
  const path = join(ROOT, relative);
  const current = existsSync(path) ? readFileSync(path, "utf8") : "";
  const before = current.split("\n"), after = rendered.split("\n");
  const beforeLines = new Set(before), afterLines = new Set(after);
  const added = after.filter((line) => !beforeLines.has(line)).length, removed = before.filter((line) => !afterLines.has(line)).length;
  console.log(`[schema-catalog-regen] ${relative}: ${current === rendered ? "current" : `stale — ${before.length} → ${after.length} lines, ~${added} added / ~${removed} removed`}`);
  if (relative.endsWith(".json") && current !== rendered) {
    const was = scopesOf(current), now = scopesOf(rendered);
    const gone = Object.keys(was).filter((scope) => !(scope in now)), fresh = Object.keys(now).filter((scope) => !(scope in was));
    const changed = Object.keys(now).filter((scope) => scope in was && JSON.stringify(was[scope]) !== JSON.stringify(now[scope]));
    console.log(JSON.stringify({ scopes: Object.keys(now).length, added: fresh, removed: gone, changed: changed.length, changedScopes: changed.slice(0, 60) }, null, 1));
  }
  const preview = join(STATE, `preview-${relative.split("/").at(-1)}`);
  writeFileSync(preview, rendered);
  if (apply && current !== rendered) {
    if (existsSync(path)) {
      mkdirSync(dirname(join(backup, relative)), { recursive: true });
      copyFileSync(path, join(backup, relative));
    }
    writeFileSync(path, rendered);
    console.log(`[schema-catalog-regen] wrote ${relative} (backup ${backup})`);
  }
}
console.log(`[schema-catalog-regen] ${inventory.modules.length} modules, ${Object.keys(inventory.catalog.scopes).length} scopes, ${inventory.diagnostics.length} diagnostics, ${Math.round(performance.now() - started)} ms`);
