import { describe, expect, it } from "bun:test";
import { resolve } from "node:path";
import { clearColorResolveCache, resolveColorHex, resolveColorRgba, resolveSemanticColorHex } from "./index.ts";
import { puzzle3dLockedExampleMeshBasenames, puzzle3dMeshBasenamesInJson } from "../vite-elements-assets.ts";
import { PLAYGROUND_LOCKED_EXAMPLE_ENV } from "../../../repo/lib/js/index.ts";

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

describe("puzzle3d mesh build helpers", () => {
  it("collects mesh basenames from fixture JSON", () => {
    const basenames = puzzle3dMeshBasenamesInJson({
      objects: [{ meshUrl: "/mesh/hexagonal-cut-concrete-forest-left.glb" }],
      meta: { kindCatalogs: { objects: [{ meshUrl: "/mesh/capsule_J.glb" }] } },
    });
    expect([...basenames].sort()).toEqual(["capsule_J.glb", "hexagonal-cut-concrete-forest-left.glb"]);
  });

  it("returns only concrete forest glbs when fixture is locked", () => {
    const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
    try {
      process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
      const basenames = puzzle3dLockedExampleMeshBasenames(repoRoot);
      expect(basenames?.has("hexagonal-cut-concrete-forest-left.glb")).toBe(true);
      expect(basenames?.has("hexagonal-cut-concrete-forest-right.glb")).toBe(true);
      expect(basenames?.has("capsule_J.glb")).toBe(false);
      expect(basenames?.has("placeholder.glb")).toBe(true);
    } finally {
      if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
    }
  });
});
