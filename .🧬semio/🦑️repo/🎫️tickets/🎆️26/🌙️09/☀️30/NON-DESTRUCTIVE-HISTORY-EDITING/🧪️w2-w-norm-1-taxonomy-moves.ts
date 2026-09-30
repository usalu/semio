/**
 * 🧪️ W2-W norm-1: the taxonomy inventory of one scope — every path whose canonical (normalized) spelling differs from
 * its source spelling, and every violation — without the move-reference planner, which aborts repo-wide on an unrelated
 * frozen-evidence digest.
 *
 *     bun 🧪️w2-w-norm-1-taxonomy-moves.ts <scope>
 */
import { resolve } from "node:path";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const scope = process.argv[2]!;
const inventory = inventoryTaxonomy({ repoRoot: root, scope });
const moves = inventory.entries.filter((entry) => entry.normalizedPath !== entry.sourcePath).map((entry) => ({ from: entry.sourcePath, to: entry.normalizedPath }));
const violations = inventory.violations.map(({ code, path, message }) => ({ code, path, message }));
console.log(JSON.stringify({ scope, moves, violations }, null, 2));
