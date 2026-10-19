// 🔬 (tax3 copy) Inventories one taxonomy scope without planning reference edits and prints JSON {scope, violations, moves}. usage: bun inventory.ts <scope> <out.json>
import {writeFileSync} from "node:fs";
import {inventoryTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔍️inventory-planning/🟦️.ts";
const scope = process.argv[2]!;
let last = "";
const inventory = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", ...(scope === "." ? {} : {scope}), progress: (p: any) => { const k = `${p.operation}/${p.phase}`; if (k !== last) { last = k; console.error(`[progress] ${k} ${p.current}/${p.total}`); } }} as any);
const moves = inventory.entries.filter((e) => e.sourcePath !== e.normalizedPath).map((e) => ({from: e.sourcePath, to: e.normalizedPath, nodeKind: e.nodeKind, fileKind: e.fileKind}));
writeFileSync(process.argv[3]!, JSON.stringify({scope, entries: inventory.entries.length, violations: inventory.violations, moves}, null, 1));
console.error(`[done] entries=${inventory.entries.length} violations=${inventory.violations.length} moves=${moves.length}`);
