/** 📇️ S5-GATES: a dry run of the central `bun ./📜️script.ts schema generate` + `schema docs` — renders the derived scope catalog and
 * its index document from the schema modules on disk with the generator's own functions, writes NOTHING into the tree, and prints
 * what the real run would change: bytes and lines of both files, and the scopes added, removed and changed (with the changed keys).
 * `bun 🧪️s5-gates-schema-generate-dry-run.ts <scratch directory>` keeps the two rendered files there. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const repoRoot = decodeURIComponent(new URL("../../../../../../../", import.meta.url).pathname).replace(/\/$/u, "");
const scratch = process.argv[2];
if (scratch === undefined || scratch === "") throw new Error("usage: bun 🧪️s5-gates-schema-generate-dry-run.ts <scratch directory>");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inventory = inventorySchemaScopes(repoRoot);
const rendered = { [`${library}/🔣️schema-catalog.json`]: renderSchemaCatalog(inventory.catalog), [`${library}/📓️schema-catalog.md`]: renderSchemaCatalogDocument(inventory.catalog) };
mkdirSync(scratch, { recursive: true });
for (const [path, text] of Object.entries(rendered)) {
  const current = readFileSync(join(repoRoot, path), "utf8");
  writeFileSync(join(scratch, path.slice(path.lastIndexOf("/") + 1)), text);
  console.log(`[schema generate dry-run] ${path}: ${current === text ? "CURRENT" : "STALE"} — ${current.length} → ${text.length} chars, ${current.split("\n").length} → ${text.split("\n").length} lines`);
}
const tracked = (JSON.parse(readFileSync(join(repoRoot, `${library}/🔣️schema-catalog.json`), "utf8")) as { scopes: Record<string, unknown> }).scopes;
const next = inventory.catalog.scopes as unknown as Record<string, Record<string, unknown>>;
const added = Object.keys(next).filter((id) => !(id in tracked));
const removed = Object.keys(tracked).filter((id) => !(id in next));
const changed = Object.keys(next).filter((id) => id in tracked && JSON.stringify(tracked[id]) !== JSON.stringify(next[id]));
const keys = new Map<string, number>();
for (const id of changed) for (const key of new Set([...Object.keys(next[id]!), ...Object.keys(tracked[id] as Record<string, unknown>)])) if (JSON.stringify((tracked[id] as Record<string, unknown>)[key]) !== JSON.stringify(next[id]![key])) keys.set(key, (keys.get(key) ?? 0) + 1);
const owner = (id: string): string => id.split(".").slice(0, id.startsWith("s.") || id.startsWith("app.") ? 2 : 1).join(".");
const tally = (ids: readonly string[]): string => [...ids.reduce((counts, id) => counts.set(owner(id), (counts.get(owner(id)) ?? 0) + 1), new Map<string, number>())].sort((left, right) => right[1] - left[1]).map(([name, count]) => `${name} ${count}`).join(", ");
console.log(`[schema generate dry-run] scopes ${Object.keys(tracked).length} → ${Object.keys(next).length}: +${added.length} added, -${removed.length} removed, ~${changed.length} changed (${[...keys].map(([key, count]) => `${key} ${count}`).join(", ")}); ${inventory.diagnostics.length} generator diagnostics`);
console.log(`[schema generate dry-run] added by owner: ${tally(added)}`);
console.log(`[schema generate dry-run] removed by owner: ${tally(removed)}`);
console.log(`[schema generate dry-run] changed by owner: ${tally(changed)}`);
