/** 🧭️ hr2 taxonomy inventory probe: runs the real inventoryTaxonomy on one scope and writes its violations. Usage: bun inv.ts <scope> <out.json> */
import { writeFileSync } from "node:fs";
import { inventoryTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const [scope, out] = process.argv.slice(2);
const started = Date.now();
const inventory = inventoryTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", scope: scope === "." ? undefined : scope });
const counts: Record<string, number> = {};
for (const v of inventory.violations) counts[v.code] = (counts[v.code] ?? 0) + 1;
console.log(`scope=${scope} entries=${inventory.entries.length} violations=${inventory.violations.length} ms=${Date.now() - started}`);
console.log(JSON.stringify(counts));
writeFileSync(out!, JSON.stringify({ scope, counts, violations: inventory.violations }, null, 1));
