/** ⚖️ Old-vs-new taxonomy inventory identity: inventories each scope with the current module and with the pre-change snapshot back to back in
 * one process and byte-compares the canonical inventory (entries, references, violations, digests).
 * argv: <repoRoot> <oldNormalizationModule> <scope>... */
import { createHash } from "node:crypto";

const [repoRoot, oldModule, ...scopes] = process.argv.slice(2);
type Inventory = { entries: unknown[]; violations: unknown[]; inventoryDigest: string; sourceTreeDigest: string };
const current = await import(`${repoRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`) as { inventoryTaxonomy: (options: { repoRoot: string; scope: string }) => Inventory };
const previous = await import(oldModule!) as typeof current;
const digest = (inventory: Inventory): string => createHash("sha256").update(JSON.stringify({ entries: inventory.entries, violations: inventory.violations, inventoryDigest: inventory.inventoryDigest, sourceTreeDigest: inventory.sourceTreeDigest })).digest("hex");
for (const scope of scopes) {
  let start = performance.now();
  const after = current.inventoryTaxonomy({ repoRoot: repoRoot!, scope });
  const newSeconds = (performance.now() - start) / 1000;
  start = performance.now();
  const before = previous.inventoryTaxonomy({ repoRoot: repoRoot!, scope });
  const oldSeconds = (performance.now() - start) / 1000;
  const same = digest(after) === digest(before);
  console.log(`[DEBUG] ${same ? "identical" : "DIFFERENT"}\t${scope}\tentries=${after.entries.length}\tviolations=${after.violations.length}\tinventoryDigest=${after.inventoryDigest.slice(0, 12)}/${before.inventoryDigest.slice(0, 12)}\tnew=${newSeconds.toFixed(1)}s\told=${oldSeconds.toFixed(1)}s`);
  if (!same) {
    const left = new Map((before.entries as { sourcePath: string }[]).map((entry) => [entry.sourcePath, JSON.stringify(entry)]));
    let shown = 0;
    for (const entry of (after.entries as { sourcePath: string; nodeKind: string }[]).filter((row) => row.nodeKind !== "directory")) if (left.get(entry.sourcePath) !== JSON.stringify(entry) && shown++ < 8) console.log(`[DEBUG]   differs ${entry.sourcePath}\n    old ${(left.get(entry.sourcePath) ?? "absent").slice(0, 400)}\n    new ${JSON.stringify(entry).slice(0, 400)}`);
  }
}
