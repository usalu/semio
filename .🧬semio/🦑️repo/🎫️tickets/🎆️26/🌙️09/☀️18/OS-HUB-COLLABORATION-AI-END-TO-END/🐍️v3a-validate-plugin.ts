#!/usr/bin/env bun
/** 🚦️ V3a fast per-plugin taxonomy-tree audit — calls the registry's own `validateTaxonomyTree`
 * directly so a before/after count for one plugin costs seconds instead of the 4.5 min full `check`.
 * Usage: bun 🐍️v3a-validate-plugin.ts [pluginDirName …]   (no args = every plugin) */
import { existsSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { validateTaxonomyTree } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const wanted = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const verbose = process.argv.includes("--verbose");
const plugins = readdirSync(pluginsRoot)
  .filter((n) => !n.startsWith(".") && statSync(join(pluginsRoot, n)).isDirectory())
  .filter((n) => wanted.length === 0 || wanted.some((w) => n.includes(w)))
  .sort();

let total = 0;
for (const plugin of plugins) {
  const root = join(pluginsRoot, plugin);
  if (!existsSync(root)) continue;
  const findings = validateTaxonomyTree(root, plugin);
  total += findings.length;
  const schemaLane = findings.filter((f) => /is missing (🎚️config|👥️presence)\/🧬️schema/.test(f)).length;
  const unreachable = findings.filter((f) => f.includes("is not reachable from Cargo manifest")).length;
  console.log(`${String(findings.length).padStart(5)} ${plugin}  (schema-lane ${schemaLane}, unreachable ${unreachable})`);
  if (verbose) for (const f of findings) console.log(`      - ${f}`);
}
console.log(`TOTAL ${total} over ${plugins.length} plugin(s)`);
