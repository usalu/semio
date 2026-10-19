// 🔬 [DEBUG] prints inventory entries under a path substring. usage: bun dbg-entries.ts <scope> <substring>
import {inventoryTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔍️inventory-planning/🟦️.ts";
const [scope, sub] = process.argv.slice(2);
const inv = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", scope} as any);
for (const e of inv.entries as any[]) if (e.sourcePath.includes(sub!)) console.log(JSON.stringify({s: e.sourcePath.slice(-90), n: e.normalizedPath.slice(-90), k: e.nodeKind, dk: e.directoryKind ?? e.kindId ?? e.kind, v: e.violations?.map((x: any) => x.code)}));
