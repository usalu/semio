// 🔬 Runs the taxonomy inventory over the library scope of a mirror (its own copy of the engine and taxonomy). usage: bun inv.ts <mirror> <out.json> [scope]
import {writeFileSync} from "node:fs";
import {resolve} from "node:path";
import {LIB} from "./vfs.ts";
const mirror = resolve(process.argv[2]!), scope = process.argv[4] ?? LIB;
const {inventoryTaxonomy} = await import(`${mirror}/${LIB}/🧹️normalization/🔍️inventory-planning/🟦️.ts`);
const inventory = inventoryTaxonomy({repoRoot: mirror, taxonomyPath: `${mirror}/${LIB}/🔣️taxonomy.json`, scope} as any);
writeFileSync(process.argv[3]!, JSON.stringify({scope, entries: inventory.entries.length, violations: inventory.violations}, null, 1));
console.error(`[done] entries=${inventory.entries.length} violations=${inventory.violations.length}`);
