/** ⏱️ Times `verify taxonomy` phase by phase on an explicit scope: inventory vs plan, with the
 * progress stream sampled so a long phase is visible while it runs instead of only at the end. */

import { inventoryTaxonomy, planTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
import { execFileSync } from "node:child_process";

const repoRoot = process.argv[2]!;
const scope = process.argv[3] && process.argv[3] !== "-" ? process.argv[3] : undefined;
const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: repoRoot, encoding: "utf8" }).trim();

let last = Date.now();
const seen = new Map<string, { count: number; firstAt: number; lastAt: number }>();
const start = Date.now();
const progress = (event: { operation: string; phase: string; current: number; total: number; path?: string }): void => {
  const key = `${event.operation}/${event.phase}`;
  const row = seen.get(key) ?? { count: 0, firstAt: Date.now(), lastAt: Date.now() };
  row.count++;
  row.lastAt = Date.now();
  seen.set(key, row);
  if (Date.now() - last > 5000) {
    last = Date.now();
    console.error(`[${((Date.now() - start) / 1000).toFixed(1)}s] ${key} ${event.current}/${event.total} ${event.path ?? ""}`);
  }
};

const t0 = Date.now();
const inventory = inventoryTaxonomy({ repoRoot, ...(scope ? { scope } : {}), progress });
const t1 = Date.now();
console.log(`scope=${scope ?? "(whole repo)"} inventory=${((t1 - t0) / 1000).toFixed(1)}s entries=${inventory.entries.length} violations=${inventory.violations.length}`);
if (inventory.entries.length === 0) throw new Error("inventory discovered zero entries");
const plan = planTaxonomy(inventory, { baselineCommit: head, excludedTreeDigests: [], progress });
const t2 = Date.now();
console.log(`plan=${((t2 - t1) / 1000).toFixed(1)}s moves=${plan.moves.length} edits=${plan.edits.length} removals=${plan.evidenceRemovals.length} regenerations=${plan.regenerations.length} unresolved=${plan.unresolved.length}`);
console.log("phase samples (count, span s):");
for (const [key, row] of [...seen.entries()].sort((left, right) => right[1].lastAt - right[1].firstAt - (left[1].lastAt - left[1].firstAt))) {
  console.log(`  ${key} count=${row.count} span=${((row.lastAt - row.firstAt) / 1000).toFixed(1)}s`);
}
