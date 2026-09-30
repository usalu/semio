/** 🔭️ Inventory-only taxonomy probe: the scope's violations and moves without the repo-wide reference plan. */
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const [scope, ...filters] = process.argv.slice(2);
if (!scope) throw new Error("usage: bun 🧪️w3-gltf-taxonomy-inventory.ts <scope> [code-filter...]");
const inventory = inventoryTaxonomy({ repoRoot: process.cwd(), scope });
const rows = inventory.violations.filter((row) => filters.length === 0 || filters.includes(row.code));
const moves = inventory.entries.filter((entry) => entry.normalizedPath !== entry.sourcePath);
const counts = new Map<string, number>();
for (const row of inventory.violations) counts.set(row.code, (counts.get(row.code) ?? 0) + 1);
for (const row of rows) console.log(`${row.severity} ${row.code} ${row.path}: ${row.message}`);
for (const move of moves.slice(0, 20)) console.log(`move ${move.sourcePath} -> ${move.normalizedPath}`);
console.log(`[inventory] scope=${scope} entries=${inventory.entries.length} violations=${inventory.violations.length} moves=${moves.length}`);
for (const [code, count] of [...counts].sort()) console.log(`[inventory] ${code}=${count}`);
