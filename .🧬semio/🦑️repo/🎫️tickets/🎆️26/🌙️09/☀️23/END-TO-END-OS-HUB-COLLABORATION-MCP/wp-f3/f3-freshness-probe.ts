/** ⏱️ F3 — where the serve-start freshness pass spends its synchronous time: catalog projection reads, and per plugin
 * whether the staged stat index exists, the source file count, and the sync time of each step. usage: bun f3-freshness-probe.ts */
import { existsSync } from "node:fs";
import { join } from "node:path";
const DEV = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const act = await import(`${DEV}/♻️activation/🟦️.ts`);
const view = await import(`${DEV}/../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts`);
const plan = await import(`${DEV}/../🔌️plugin/🏗️build/📋️plan/🟦️.ts`);
const { getWorkspaceRoot } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
const { moduleDirectoryName } = await import(`${DEV}/../🔌️plugin/📇️registry/📦️deployment/🟦️.ts`);
const repoRoot = getWorkspaceRoot();
let t = performance.now();
const lap = () => { const now = performance.now(); const d = now - t; t = now; return d.toFixed(0); };
const projection = view.readGeneratedCatalogProjection();
console.log(`readGeneratedCatalogProjection ${lap()} ms (${projection.entries.length} entries)`);
const selected = new Set(view.filterProjectedPluginRegistry(projection, plan.resolveCatalogFilterPluginId("s")).map((entry: { pluginId: string }) => entry.pluginId));
console.log(`filterProjectedPluginRegistry ${lap()} ms (${selected.size})`);
const moduleRoot = act.pluginModulesRoot("dev");
const rows: string[] = [];
let totalFiles = 0;
for (const target of projection.entries.filter((entry: { pluginId: string }) => selected.has(entry.pluginId))) {
  const moduleDirectory = join(moduleRoot, moduleDirectoryName(target.pluginId));
  const sourceRoot = join(repoRoot, target.cratePath, "..", "..");
  t = performance.now();
  const index = act.readStagedSourceStatIndex(moduleDirectory);
  const indexMs = lap();
  const files = await act.listComponentSourceFilesAsync(sourceRoot);
  const listMs = lap();
  totalFiles += files.length;
  rows.push(`${target.pluginId.padEnd(14)} index=${index ? `${index.files.length}f/${indexMs}ms` : "MISSING"} files=${files.length} list=${listMs}ms staged=${existsSync(moduleDirectory)}`);
}
console.log(rows.join("\n"));
console.log(`total source files ${totalFiles}`);
