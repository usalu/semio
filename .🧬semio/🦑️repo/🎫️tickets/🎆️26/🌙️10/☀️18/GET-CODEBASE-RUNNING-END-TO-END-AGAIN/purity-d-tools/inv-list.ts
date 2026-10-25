import { inventoryTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔍️inventory-planning/🟦️.ts";
const inv = inventoryTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", scope: process.argv[2]!, workers: 1 });
const want = new Set(process.argv.slice(3));
for (const e of inv.entries) for (const v of e.violations) if (want.size === 0 || want.has(v.code)) console.log(v.code + "\t" + e.sourcePath.replace(/^🧰️framework\/🛍️products\/🦑️repo\/🔨️modules\//, "M/") + "\t" + v.message.slice(0, 120));
