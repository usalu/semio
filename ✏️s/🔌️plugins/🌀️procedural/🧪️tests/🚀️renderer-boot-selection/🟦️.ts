import { expect, test } from "bun:test";
import { parseSurfaceAppId, resolvePlaygroundBoot, type PluginCatalog } from "@semio-tech/framework";
import { resolveBootPrimaryAppV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import corpus from "../../🧫️fixtures/🚀️renderer-boot-selection/🔣️.json";

test("procedural owns its dependency ordered renderer boot declaration", () => {
  const catalog: PluginCatalog = {
    plugins: corpus.catalog.plugins.map(entry => ({ ...entry, wasmOut: `${entry.pluginId}.wasm`, role: "plugin", contributes: [], consumes: [] })),
    extensions: [], hosts: [], playgrounds: corpus.catalog.playgrounds,
    moduleUrl: pluginId => `${pluginId}.wasm`, extensionModuleUrl: pluginId => `${pluginId}.wasm`,
  };
  const boot = resolvePlaygroundBoot(catalog, "generation3d");
  const original = corpus.cases.find(entry => entry.id === "dependency-sorts-first")!;
  expect(boot.plugins.map(entry => entry.pluginId)).toEqual(original.programs.map(entry => entry.pluginId));
  expect(boot.defaultAppId).toBe(original.expected!.appId);
  expect(parseSurfaceAppId(boot.defaultAppId!).dialect.artifactKind).toBe("s.procedural.generation3d");
});

test("procedural retains all original renderer role projection cases", () => {
  expect(corpus.cases).toHaveLength(9);
  for (const row of corpus.cases) {
    if (row.expected === null) {
      expect(row.programs.every(program => program.pluginId !== "procedural")).toBe(true);
      continue;
    }
    const program = row.programs[row.expected.index];
    const apps = program.appIds.map(id => ({ id, ...parseSurfaceAppId(id) }));
    const requested = corpus.catalog.playgrounds.find(entry => entry.variant === row.variant)?.app;
    const selected = resolveBootPrimaryAppV1(apps, undefined, requested, "role" in row ? row.role as "editor" | "viewer" : "editor");
    expect(selected?.id).toBe(row.expected.appId);
  }
});
