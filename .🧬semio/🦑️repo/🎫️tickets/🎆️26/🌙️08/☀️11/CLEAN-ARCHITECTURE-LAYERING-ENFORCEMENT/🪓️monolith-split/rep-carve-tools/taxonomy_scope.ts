// 🧭️ Runs the repository taxonomy inventory over the replication module only and prints violations (analysis only).
import { inventoryTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const scope = process.argv[2]!;
const inventory = inventoryTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", scope, workers: 1 });
const by = new Map<string, number>();
for (const v of inventory.violations) by.set(v.code, (by.get(v.code) ?? 0) + 1);
console.log("entries", inventory.entries.length, "violations", inventory.violations.length, JSON.stringify([...by]));
const own = ["✏️edit", "🕹️interaction", "👥️presence", "🆔️ids", "🌐️foreign", "🧾️policy", "🔬️facade", "🏛️crate-partition", "🧬️schema/🏛️", "📦️packages/🦀️rust"];
for (const v of inventory.violations) console.log(v.code, v.path, String(v.message).slice(0, 120));
