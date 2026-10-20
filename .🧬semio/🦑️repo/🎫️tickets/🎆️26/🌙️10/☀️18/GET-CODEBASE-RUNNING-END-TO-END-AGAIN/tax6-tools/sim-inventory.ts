// 🔬 Runs the inventory from the patched scratch copy of the library against the live tree. usage: TAX6_SIM=<scratch name> bun sim-inventory.ts <scope> <out.json>
import {writeFileSync} from "node:fs";
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const L = `${C}/🗑️generated/${process.env.TAX6_SIM ?? "tax6-sim"}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const {inventoryTaxonomy} = await import(`/Users/ueli/Documents/semio/${L}/🧹️normalization/🔍️inventory-planning/🟦️.ts`);
const scope = process.argv[2]!;
const inventory = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: `${L}/🔣️taxonomy.json`, ...(scope === "." ? {} : {scope})} as any);
const moves = inventory.entries.filter((e: any) => e.sourcePath !== e.normalizedPath).map((e: any) => ({from: e.sourcePath, to: e.normalizedPath, nodeKind: e.nodeKind, fileKind: e.fileKind}));
writeFileSync(process.argv[3]!, JSON.stringify({scope, entries: inventory.entries.length, violations: inventory.violations, moves}, null, 1));
console.error(`[done] entries=${inventory.entries.length} violations=${inventory.violations.length} moves=${moves.length}`);
