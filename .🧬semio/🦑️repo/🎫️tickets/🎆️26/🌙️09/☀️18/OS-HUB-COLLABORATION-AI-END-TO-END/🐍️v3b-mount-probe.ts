#!/usr/bin/env bun
/** 🔬️ V3b probe: why one leaf is (or is not) reachable, from the gate's own module graph.
 *
 * Rebuilds `inspectRustModuleGraph` over one plugin root exactly as `validateRustTaxonomyMounts` does
 * (every `.rs` under the root plus every `Cargo.toml`), then prints the contexts of the leaf under
 * question and of each file on the chain that should reach it.
 *
 * Usage: `bun 🐍️v3b-mount-probe.ts <pluginDirName> <leafRelPath> [<ancestorRelPath> …]`
 */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts, isDiscoverySkipDirectory } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const pluginRoot = join(repoRoot, "✏️s/🔌️plugins", process.argv[2]!);
const leafRel = process.argv[3]!;

const sources: string[] = [];
const manifests: string[] = [];
function walk(dir: string): void {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || isDiscoverySkipDirectory(name)) continue;
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      walk(path);
      continue;
    }
    if (name === "Cargo.toml") manifests.push(relative(pluginRoot, path).replaceAll("\\", "/"));
    if (name.endsWith(".rs")) sources.push(relative(pluginRoot, path).replaceAll("\\", "/"));
  }
}
walk(pluginRoot);
if (sources.length === 0) throw new Error("no Rust sources under the plugin root");
console.log(`sources=${sources.length} manifests=${manifests.length}`);

const graph = inspectRustModuleGraph([...sources, ...manifests], (path) => readFileSync(join(pluginRoot, path), "utf8"), { strictManifests: true });
console.log(`leaf in source set: ${sources.includes(leafRel)}`);
for (const path of process.argv.slice(3)) {
  const rows = graph.contexts.get(path) ?? [];
  console.log(`\n=== ${path} — ${rows.length} context(s) ===`);
  for (const row of rows) console.log(`  crateRoot=${row.crateRoot} manifest=${row.manifestPath} modulePath=${row.modulePath.join("::")} sourceScope=${row.sourceScope.join("::")} moduleBase=${row.moduleBase}`);
  try {
    const facts = inspectRustModuleGraphFacts(readFileSync(join(pluginRoot, path), "utf8"));
    for (const module of facts.modules) console.log(`  mod ${module.name} inline=${module.inline} conditional=${module.conditional ?? false} modulePath=${module.modulePath.join("::")} pathTarget=${module.pathTarget}`);
  } catch (error) {
    console.log(`  (unreadable: ${(error as Error).message})`);
  }
}
const collisions = [...graph.targets].filter(([, target]) => target === leafRel);
console.log(`\n=== target keys resolving to the leaf: ${collisions.length} ===`);
for (const [key, target] of collisions) console.log(`  ${key.replace("\0", " :: ")} -> ${target}`);
