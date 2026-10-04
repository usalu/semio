import { join } from "node:path";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../..");
for (const scope of process.argv.slice(2)) {
  const inventory = inventoryTaxonomy({ repoRoot: root, scope });
  console.log(`[DEBUG] ${scope}: entries=${inventory.entries.length} violations=${inventory.violations.length}`);
  for (const violation of inventory.violations) console.log(`[DEBUG] ${violation.code} ${violation.path}: ${violation.message}`);
  for (const entry of inventory.entries.slice(0, 3)) console.log(`[DEBUG] entry ${JSON.stringify(entry).slice(0, 400)}`);
}
