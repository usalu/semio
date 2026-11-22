import assert from "node:assert/strict";
import { resolve } from "node:path";
import { fetchPackageDescriptor, PlaygroundBootPlanner } from "@semio-tech/framework";
import { componentPackageReceiptSource } from "@semio-tech/framework-plugin/receipt";
import { admitWgpuPluginModules, assertWgpuPluginPlan, prepareWgpuPluginModules } from "@semio-tech/framework-renderer-wgpu/plugin-modules";

const base = process.argv[2] ?? "http://127.0.0.1:6041/";
const variant = "gis2d";
const { composeSpecificOsCatalogV1 } = await import(resolve("✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts"));
const { rows, catalog } = composeSpecificOsCatalogV1(base, {
  maxBytes: 2_097_152,
  maxRows: 512,
  maxEdges: 4_096,
  maxWork: 65_536,
  deadlineMs: performance.now() + 30_000,
  now: () => performance.now(),
  cancelled: () => false,
  progress: () => {},
});
const modules = admitWgpuPluginModules(
  rows.targets.map(({ pluginId, moduleUrl }) => ({ pluginId, moduleUrl })),
  base,
);
const reads: string[] = [];
const prepared = await prepareWgpuPluginModules(catalog, modules, componentPackageReceiptSource, {
  bootVariant: variant,
  readDescriptor: async (...request) => {
    reads.push(request[0]);
    return fetchPackageDescriptor(...request);
  },
});
const plan = new PlaygroundBootPlanner(prepared.catalog, variant).finish();
assertWgpuPluginPlan(plan, prepared, "variant");
assert(modules.length > 1);
assert.deepEqual(reads, ["gis", "stdio"]);
assert.deepEqual(plan.plugins.map(plugin => plugin.pluginId), ["stdio", "gis"]);
assert.equal(plan.defaultAppId, "s.gis.gismap@1/*#editor");
const sha256 = prepared.packages.get("gis")?.componentSha256;
assert.match(sha256 ?? "", /^[a-f0-9]{64}$/);
console.log(JSON.stringify({ installed: modules.length, reads, app: plan.defaultAppId, sha256 }));
