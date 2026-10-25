import { inventoryTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔍️inventory-planning/🟦️.ts";
const scope = process.argv[2]!;
const started = Date.now();
const inv = inventoryTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", scope, workers: 1 });
const counts: Record<string, number> = {};
let moves = 0;
for (const e of inv.entries) { if (e.normalizedPath !== e.sourcePath) moves++; for (const v of e.violations) counts[v.code] = (counts[v.code] ?? 0) + 1; }
for (const e of inv.entries) if (e.normalizedPath !== e.sourcePath) console.log("MOVE", e.sourcePath, "->", e.normalizedPath);
console.log("entries", inv.entries.length, "moves", moves, "ms", Date.now() - started, JSON.stringify(counts), "global", JSON.stringify(inv.violations.map((v) => v.code).slice(0, 5)));
