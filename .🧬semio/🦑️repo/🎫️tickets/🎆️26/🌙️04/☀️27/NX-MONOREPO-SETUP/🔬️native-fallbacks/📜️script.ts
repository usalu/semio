import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { join, resolve, dirname, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const graph = JSON.parse(readFileSync(join(root, ".nx/workspace-data/project-graph.json"), "utf8"));
const { cacheInternals } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
const rows = [];
for (const [name, node] of Object.entries(graph.nodes) as any[]) {
  const project = node.data, directory = join(root, project.root), manifest = join(directory, "Cargo.toml");
  if (!existsSync(manifest) || !project.namedInputs?.nativeSources?.includes("{projectRoot}/**/*")) continue;
  const cargo = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  if (!cargo.package) continue;
  const build = cargo.package.build === false ? undefined : resolve(directory, typeof cargo.package.build === "string" ? cargo.package.build : "build.rs");
  let reason = build && existsSync(build) ? "Custom build script: " + relative(root, build) : "No discovered entrypoints";
  const entries = [cargo.lib?.path ?? "src/lib.rs", "src/main.rs", ...(cargo.bin ?? []).map((target: any) => target.path)].filter(Boolean).map((path) => resolve(directory, path)).filter(existsSync);
  if ((!build || !existsSync(build)) && entries.length) {
    try { cacheInternals.rustSourceFiles(entries, directory); reason = "Fallback from an automatically discovered entry or input outside this diagnostic subset"; }
    catch (error) { reason = String(error.message).replaceAll(root + "/", ""); }
  }
  rows.push({ project: name, root: project.root, reason });
}
writeFileSync(join(ticket, "🗑️generated/native-fallbacks.json"), JSON.stringify(rows, null, 2));
writeFileSync(join(ticket, "📓️native-fallbacks.md"), "# Conservative Native Source Inputs\n\nThe current graph retains conservative domain inputs for " + rows.length + " concrete Cargo packages. These reasons must be resolved before claiming minimal invalidation throughout the real native dependency graph.\n\n| Project | Cause |\n| --- | --- |\n" + rows.map((row) => "| " + row.project + " | " + row.reason.replaceAll("|", "\\|") + " |").join("\n") + "\n");
console.log("[DEBUG] Recorded " + rows.length + " native source fallback reasons");
