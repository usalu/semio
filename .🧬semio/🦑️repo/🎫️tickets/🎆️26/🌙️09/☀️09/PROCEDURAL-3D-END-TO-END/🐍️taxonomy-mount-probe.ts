/** 🔎️ Probe: re-runs `validateRustTaxonomyMounts`'s exact reachability rule over one plugin root,
 * once with only the plugin manifest (what `📇️registry/📜️script.ts:1163` feeds it) and once with
 * every nested Cargo manifest under the same root, to separate leaves that no crate mounts from
 * leaves the single-manifest input cannot see. Read-only. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { inspectRustModuleGraph } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const pluginRoot = process.argv[2] ?? "✏️s/🔌️plugins/🌀️procedural";
const sources: string[] = [];
const manifests: string[] = [];
const walk = (dir: string): void => {
  for (const name of readdirSync(dir)) {
    if (name.startsWith(".") || name === "target" || name === "node_modules" || name === "dist") continue;
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      walk(path);
      continue;
    }
    if (name.endsWith(".rs")) sources.push(relative(pluginRoot, path).replaceAll("\\", "/"));
    if (name === "Cargo.toml") manifests.push(relative(pluginRoot, path).replaceAll("\\", "/"));
  }
};
walk(pluginRoot);
const components = sources.filter((path) => path.endsWith("/🦀️.rs") || path === "🦀️.rs" || path.endsWith("/🦀️.example.rs"));
const read = (path: string): string => readFileSync(join(pluginRoot, path), "utf8");
const unreachable = (manifestSet: readonly string[]): string[] => {
  const graph = inspectRustModuleGraph([...sources, ...manifestSet], read, { strictManifests: true });
  return components.filter((path) => !(graph.contexts.get(path) ?? []).some((context) => manifestSet.includes(context.manifestPath ?? "")));
};
const single = unreachable(["📦️packages/🦀️rust/Cargo.toml"]);
const all = unreachable(manifests);
console.log(`manifests: ${manifests.length}`, manifests.join(" "));
console.log(`components: ${components.length}`);
console.log(`unreachable(plugin manifest only): ${single.length}`);
console.log(`unreachable(all nested manifests): ${all.length}`);
for (const path of all) console.log("  ORPHAN " + path);
