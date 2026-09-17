/** @emoji 🗺️ Static GitHub Pages cannot run Vite tile-proxy middleware — ship builds must bundle cached tiles. */
export type GisMapTileServeMode = "fetch" | "bundle";

/** @emoji 🎚️ Static demonstrator ship: Switzerland through z10 (~400 tiles), not full offline play z13/z14 (~46k). */
export const DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER = 10;
export const DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR = 10;

export function demonstratorGisMapTileServeMode(viteCommand: "build" | "serve"): GisMapTileServeMode {
  if (viteCommand === "build") return "bundle";
  return process.env.GIS_MAP_TILE_SERVE_MODE === "bundle" ? "bundle" : "fetch";
}
