type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: { demonstratorGisMapTileServeMode: (command: "build" | "serve") => string; DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER: number; DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR: number },
): Promise<void> {
  const { demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR } = dependencies;
  const { describe, expect, it } = vitest;

  describe("demonstratorGisMapTileServeMode", () => {
    it("bundles map tiles for production Vite builds", () => {
      expect(demonstratorGisMapTileServeMode("build")).toBe("bundle");
    });

    it("caps static ship prefetch below full offline-play depth", () => {
      expect(DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER).toBeLessThanOrEqual(10);
      expect(DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR).toBeLessThanOrEqual(10);
    });

    it("keeps dev serve on fetch unless GIS_MAP_TILE_SERVE_MODE overrides", () => {
      const previous = process.env.GIS_MAP_TILE_SERVE_MODE;
      delete process.env.GIS_MAP_TILE_SERVE_MODE;
      expect(demonstratorGisMapTileServeMode("serve")).toBe("fetch");
      process.env.GIS_MAP_TILE_SERVE_MODE = "bundle";
      expect(demonstratorGisMapTileServeMode("serve")).toBe("bundle");
      if (previous === undefined) delete process.env.GIS_MAP_TILE_SERVE_MODE;
      else process.env.GIS_MAP_TILE_SERVE_MODE = previous;
    });
  });
}
