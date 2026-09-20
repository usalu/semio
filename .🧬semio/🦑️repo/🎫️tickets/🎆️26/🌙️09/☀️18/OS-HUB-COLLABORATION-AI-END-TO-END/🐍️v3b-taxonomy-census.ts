#!/usr/bin/env bun
/** 🚦️ V3b fast census of the `plugin taxonomy tree` family of `plugin-registry check`.
 *
 * `CheckScript` (`…/📇️registry/📽️projection/🟦️.ts:504`) reaches these findings only after rendering the
 * whole catalog, which costs tens of minutes under fleet load. The findings themselves come from
 * `validateTaxonomyTree` over `findNewContractPluginRoots`, so calling that pair directly re-measures
 * the same rows in seconds — the gate's own code, not a reimplementation.
 *
 * Usage: `bun 🐍️v3b-taxonomy-census.ts [--plugin <id>] [--family <substring>]`
 */
import { findNewContractPluginRoots, validateTaxonomyTree } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const onlyPlugin = process.argv[process.argv.indexOf("--plugin") + 1];
const onlyFamily = process.argv[process.argv.indexOf("--family") + 1];
const repoRoot = getWorkspaceRoot();
const roots = findNewContractPluginRoots(repoRoot).filter((row) => !process.argv.includes("--plugin") || row.pluginId === onlyPlugin);
if (roots.length === 0) throw new Error("no plugin roots discovered — the census would silently report zero");

const findings: string[] = [];
for (const { pluginId, pluginRoot } of roots) findings.push(...validateTaxonomyTree(pluginRoot, pluginId));
const selected = process.argv.includes("--family") ? findings.filter((row) => row.includes(onlyFamily!)) : findings;

const families = new Map<string, number>();
for (const row of findings) {
  const body = row.slice(row.indexOf(": ") + 2);
  const family = body
    .replace(/^.* is not reachable from Cargo manifest .*$/u, "unreachable-from-cargo-manifest")
    .replace(/"[^"]*"/gu, '"X"')
    .replace(/[^ ]*🦀️\.rs/gu, "PATH")
    .replace(/[^ ]*🟦️\.ts/gu, "PATH");
  families.set(family, (families.get(family) ?? 0) + 1);
}
console.log(`plugins=${roots.length} findings=${findings.length}`);
for (const [family, count] of [...families].sort((a, b) => b[1] - a[1])) console.log(`  ${String(count).padStart(5)}  ${family}`);
console.log("=== rows ===");
for (const row of selected) console.log(row);
