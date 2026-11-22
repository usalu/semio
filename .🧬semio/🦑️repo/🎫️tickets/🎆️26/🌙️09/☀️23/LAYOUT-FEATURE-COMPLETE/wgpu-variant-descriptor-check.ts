import assert from "node:assert/strict";
import { resolve } from "node:path";
import graphlib from "graphlib";
const { prepareWgpuPluginModules, assertWgpuPluginPlan } = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️plugin-modules/🟦️.ts"));
const { PlaygroundBootPlanner } = await import("@semio-tech/framework-kernel/playground-boot");
const fixture = await Bun.file(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🧩️plugin-modules/🔣️.json")).json();
const row = fixture.selection.find(row => row.id === "custom-dependency");
const catalog = {
  plugins: row.rows.map(module => ({
    ...module, role: "plugin", wasmOut: module.moduleUrl, contributes: [], consumes: [],
    dependsOn: module.dependencies?.map(edge => edge.pluginId) ?? [],
  })),
  extensions: [], hosts: [], playgrounds: [],
  moduleUrl: id => new URL(row.rows.find(module => module.pluginId === id).moduleUrl, fixture.baseUrl).href,
  extensionModuleUrl: () => { throw Error("No extensions in fixture"); },
};
const graph = new graphlib.Graph({ directed: true });
const pending = [row.variant], seen = new Set();
while (pending.length > 0) {
  const id = pending.pop();
  if (seen.has(id)) continue;
  seen.add(id);
  graph.setNode(id);
  for (const edge of row.rows.find(module => module.pluginId === id).dependencies ?? []) {
    graph.setEdge(edge.pluginId, id);
    pending.push(edge.pluginId);
  }
}
const modules = row.rows.map(({ pluginId }) => ({ pluginId, moduleUrl: catalog.moduleUrl(pluginId) }));
const fetched = [];
const receiptSource = { async read() { throw Error("Test uses admitted descriptor doubles"); } };
const readDescriptor = async (id, extra = []) => {
  if (!seen.has(id)) throw Error("Unpublished unrelated descriptor " + id);
  fetched.push(id);
  const module = row.rows.find(module => module.pluginId === id);
  return { packageId: id, componentSha256: "a".repeat(64), manifest: {
    pluginId: id, version: "1.0.0", label: id, apps: [], workflows: [], examples: [],
    dependencies: [...(module.dependencies ?? []), ...extra],
  } };
};
const prepared = await prepareWgpuPluginModules(catalog, modules, receiptSource, { bootVariant: row.variant, readDescriptor: id => readDescriptor(id) });
assert.deepEqual([...fetched].sort(), [...row.expected].sort());
const plan = new PlaygroundBootPlanner(prepared.catalog, row.variant).finish();
assertWgpuPluginPlan(plan, prepared, "variant");
assert.deepEqual(plan.plugins.map(module => module.pluginId), graphlib.alg.topsort(graph));
assert.deepEqual(plan.plugins.map(module => module.pluginId), row.expected);
const divergent = await prepareWgpuPluginModules(catalog, modules, receiptSource, {
  bootVariant: row.variant,
  readDescriptor: id => readDescriptor(id, id === row.variant ? [{ pluginId: "absent" }] : []),
});
assert.throws(() => assertWgpuPluginPlan(new PlaygroundBootPlanner(divergent.catalog, row.variant).finish(), divergent, "variant"), /dependency graph/);
console.log("Verified descriptor requests and dependency closure against the portable fixture and graphlib.");
