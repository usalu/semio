// 🔬 Runs the inventory from a patched scratch copy of the library against the live tree. usage: TAX5_SIM=<scratch name> bun sim-inventory.ts <scope> <out.json>
import {writeFileSync} from "node:fs";
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const L = process.env.TAX5_LIVE === "1" ? "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" : `${C}/🗑️generated/${process.env.TAX5_SIM ?? "tax5-sim"}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const {inventoryTaxonomy} = await import(`/Users/ueli/Documents/semio/${L}/🧹️normalization/🔍️inventory-planning/🟦️.ts`);
const scope = process.argv[2]!;
const inventory = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: `${L}/🔣️taxonomy.json`, ...(scope === "." ? {} : {scope})} as any);
writeFileSync(process.argv[3]!, JSON.stringify({scope, entries: inventory.entries.length, violations: inventory.violations}, null, 1));
console.error(`[done] entries=${inventory.entries.length} violations=${inventory.violations.length}`);
