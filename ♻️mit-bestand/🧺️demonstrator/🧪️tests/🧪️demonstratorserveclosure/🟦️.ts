import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { COMPONENT_MODULE_DIRECTORIES, EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { PLAYGROUND_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/playgrounds";
import { moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";
const D = import.meta.dirname + "/../../🔨️modules/🧩️runtime";
const { demonstratorActivationLanes, mergeDemonstratorActivationReceipts, demonstratorActivationComponents, DEMONSTRATOR_UNION_RECEIPT_DIRECTORY } = await import(D + "/♻️activation/🟦️.ts");
const { demonstratorRuntimeComponentIds, demonstratorRuntimeAssetSources } = await import(D + "/📦️assets/🟦️.ts");
const { demonstratorRuntimeModuleLayout, DEMONSTRATOR_RUNTIME_TARGETS } = await import(D + "/🟦️.ts");
const comps = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const laneReceipts = () => demonstratorActivationLanes().map((lane: string) => {
  const p: any = PLAYGROUND_BUILD_TARGETS.find(r => r.variant === lane)!;
  const ids = runtimeComponentClosure(comps as any, [{ id: p.pluginId, appScoped: p.app !== undefined }] as any);
  return { lane, receipt: { schema: "semio.dev.activation/v1", variant: lane, profile: "prototype", plugins: ids.map((pluginId: string) => ({ pluginId, artifactSha256: sha(pluginId), rebuiltAt: 1 })) } };
});
test("session-shaped lane closures merge into exactly the runtime union", () => {
  const merged = mergeDemonstratorActivationReceipts(laneReceipts(), demonstratorRuntimeComponentIds());
  expect(merged.plugins.map((r: any) => r.pluginId).sort()).toEqual([...demonstratorRuntimeComponentIds()].sort());
  expect(merged.variant).toBe("generator");
});
test("a dropped lane or an unhealthy-excluded row is refused", () => {
  expect(() => mergeDemonstratorActivationReceipts(laneReceipts().slice(0, 2), demonstratorRuntimeComponentIds())).toThrow(/missing/);
  const lanes = laneReceipts(); lanes[0].receipt.plugins.pop();
  expect(() => mergeDemonstratorActivationReceipts(lanes, demonstratorRuntimeComponentIds())).toThrow(/missing/);
});
test("module directory names agree across activate, vite static dirs and activation components", () => {
  const byId = new Map(comps.map(r => [r.pluginId, r as any])), ids = demonstratorRuntimeComponentIds();
  const layout = demonstratorRuntimeModuleLayout([...new Set<string>(DEMONSTRATOR_RUNTIME_TARGETS.map((r: any) => r.pluginId))]);
  const served = new Set([...layout.pluginModuleDirNames, ...layout.extensionModuleDirNames]);
  const specs = demonstratorActivationComponents("/Users/ueli/Documents/semio");
  expect(specs.map((s: any) => s.pluginId).sort()).toEqual([...ids].sort());
  for (const id of ids) {
    const a = byId.get(id).directoryName, b = moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES);
    expect(a).toBe(b);
    expect(served.has(a)).toBe(true);
    expect(specs.find((s: any) => s.pluginId === id).directoryName).toBe(a);
  }
});
test("union receipt directory is where the vite config points the plugin", () => {
  expect(DEMONSTRATOR_UNION_RECEIPT_DIRECTORY).toBe("♻️mit-bestand/🧺️demonstrator/dist/♻️activation/prototype");
  expect(demonstratorRuntimeAssetSources("/Users/ueli/Documents/semio", "prototype").some((r: any) => r.owner === "infinite:fonts")).toBe(true);
});
