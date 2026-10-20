// 🔬 Runs the inventory from the patched scratch copy of the library (🗑️generated/tax4-sim/L) against the live tree. usage: bun sim-inventory.ts <scope> <out.json>
import {writeFileSync} from "node:fs";
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tax4-sim/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const {inventoryTaxonomy} = await import(`/Users/ueli/Documents/semio/${C}/🧹️normalization/🔍️inventory-planning/🟦️.ts`);
const scope = process.argv[2]!;
const inventory = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: `${C}/🔣️taxonomy.json`, ...(scope === "." ? {} : {scope})} as any);
writeFileSync(process.argv[3]!, JSON.stringify({scope, entries: inventory.entries.length, violations: inventory.violations}, null, 1));
console.error(`[done] entries=${inventory.entries.length} violations=${inventory.violations.length}`);
