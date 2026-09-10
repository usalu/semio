/** @emoji 📏️ Measures the frame Worker's boot steps natively, in µs, so `plugin-graph` and its chunked
 * successor can be compared without a browser. Run from the repo root:
 * `bun .🧬semio/.../PROCEDURAL-3D-END-TO-END/🧪️wboot-measure.ts`. */
import { PLUGIN_CATALOG } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts";
import { PlaygroundBootPlanner, expandPluginRegistry, orderPluginRegistryEntries, resolvePlaygroundBoot, resolvePluginHostConfig, resolvePluginRegistryId } from "@semio-tech/framework";

const VARIANT = process.argv[2] ?? "generation3d";
const REPEATS = Number(process.argv[3] ?? 9);

function measure(label: string, run: () => unknown): number {
  const samples: number[] = [];
  for (let index = 0; index < REPEATS; index++) {
    const startedAt = performance.now();
    run();
    samples.push((performance.now() - startedAt) * 1000);
  }
  samples.sort((left, right) => left - right);
  const median = samples[Math.floor(samples.length / 2)]!;
  console.log(`${label}\tmedian=${median.toFixed(1)}us\tmin=${samples[0]!.toFixed(1)}us\tmax=${samples[samples.length - 1]!.toFixed(1)}us`);
  return median;
}

const registryPluginId = resolvePluginRegistryId(PLUGIN_CATALOG, VARIANT);
const hostMode = resolvePluginHostConfig(PLUGIN_CATALOG, VARIANT) !== undefined;
const rows = [...PLUGIN_CATALOG.plugins, ...PLUGIN_CATALOG.extensions];
console.log(`variant=${VARIANT} registryPluginId=${registryPluginId} hostMode=${hostMode} catalogRows=${rows.length} repeats=${REPEATS}`);

measure("plugin-graph (whole resolvePlaygroundBoot)", () => resolvePlaygroundBoot(PLUGIN_CATALOG, VARIANT));
const catalogPlugins = rows.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: target.role === "extension" ? PLUGIN_CATALOG.extensionModuleUrl(target.pluginId) : PLUGIN_CATALOG.moduleUrl(target.pluginId),
  contributes: target.contributes,
  consumes: target.consumes,
  dependencies: [],
}));
measure("  phase catalog-rows (moduleUrl x N)", () => rows.map((target) => (target.role === "extension" ? PLUGIN_CATALOG.extensionModuleUrl(target.pluginId) : PLUGIN_CATALOG.moduleUrl(target.pluginId))));
const expanded = expandPluginRegistry(catalogPlugins as never, hostMode ? undefined : registryPluginId, hostMode);
measure("  phase closure (expandPluginRegistry)", () => expandPluginRegistry(catalogPlugins as never, hostMode ? undefined : registryPluginId, hostMode));
measure("  phase order (orderPluginRegistryEntries)", () => orderPluginRegistryEntries(expanded));
console.log(`closure=${expanded.length} ordered=${orderPluginRegistryEntries(expanded).order.length}`);

console.log("--- chunked planner (worst single chunk is what the 8 ms ceiling sees) ---");
const chunkWorst: Record<string, number> = {};
for (let repeat = 0; repeat < REPEATS; repeat++) {
  const planner = new PlaygroundBootPlanner(PLUGIN_CATALOG, VARIANT);
  for (;;) {
    const stage = planner.stage();
    const startedAt = performance.now();
    const more = planner.step();
    const elapsed = (performance.now() - startedAt) * 1000;
    chunkWorst[stage] = Math.max(chunkWorst[stage] ?? 0, elapsed);
    if (!more) break;
  }
  planner.finish();
}
for (const [stage, worst] of Object.entries(chunkWorst)) console.log(`  chunk ${stage}\tworst=${worst.toFixed(1)}us`);
console.log(`  chunks=${Object.keys(chunkWorst).length} worstChunk=${Math.max(...Object.values(chunkWorst)).toFixed(1)}us`);
const plannerIds = new PlaygroundBootPlanner(PLUGIN_CATALOG, VARIANT).finish().plugins.map((entry) => entry.pluginId);
const directIds = resolvePlaygroundBoot(PLUGIN_CATALOG, VARIANT).plugins.map((entry) => entry.pluginId);
console.log(`  planner==resolve: ${JSON.stringify(plannerIds) === JSON.stringify(directIds)} plugins=${plannerIds.length}`);
