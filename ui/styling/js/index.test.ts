import { describe, expect, it } from "bun:test";
import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { clearColorResolveCache, resolveColorHex, resolveColorRgba, resolveSemanticColorHex } from "./index.ts";
import { meshCollectionVitePlugin, type PlaygroundAssetSpec } from "../vite-elements-assets.ts";

const repoRoot = resolve(import.meta.dir, "../../..");

describe("styling resolve", () => {
  it("resolveColorHex resolves palette var refs headlessly", () => {
    clearColorResolveCache();
    expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
    expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
  });

  it("resolveColorRgba returns byte tuple", () => {
    clearColorResolveCache();
    expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
  });
});

describe("puzzle3d mesh-collection asset spec", () => {
  const puzzle3dMeshSpec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
    kind: "mesh-collection",
    route: "/mesh",
    roots: ["asset/metabolism/representation", "asset/abbau-aufbau"],
    placeholder: "asset/mesh/placeholder.glb",
    filterFromExamples: true,
  };

  it("resolves kit glb roots and shared placeholder", () => {
    expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[0]!, "capsule_J.glb"))).toBe(true);
    expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.placeholder))).toBe(true);
  });

  it("registers a generic mesh-collection serve/build plugin pair", () => {
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
  });
});
