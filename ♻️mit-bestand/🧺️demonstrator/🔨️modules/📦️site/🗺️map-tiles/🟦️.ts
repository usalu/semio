import { existsSync } from "node:fs";
import { resolve } from "node:path";
import {
  GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
  mapTileCacheRoots,
  prefetchMapTiles,
} from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR } from "../🗺️tile-serve-mode/🟦️.ts";

/** @emoji ⬇️ Ensures Verfolgen's default Switzerland viewport has raster + vector tiles before `closeBundle` copies them. */
export async function prefetchDemonstratorMapTiles(repoRoot: string): Promise<void> {
  const result = await prefetchMapTiles({
    repoRoot,
    bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
    raster: true,
    vector: true,
    zMinRaster: 0,
    zMaxRaster: DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER,
    zMinVector: 0,
    zMaxVector: DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR,
    skipExisting: true,
    log: (line) => console.log(line),
  });
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const canaryVt = resolve(vt, "3/1/2.pbf");
  const canaryOsm = resolve(osm, "0/0/0.png");
  if (!existsSync(canaryVt) || !existsSync(canaryOsm)) {
    throw new Error(`[demonstrator:site] map tile cache incomplete after prefetch (vt=${existsSync(canaryVt)}, osm=${existsSync(canaryOsm)}, failed=${result.failed})`);
  }
  if (result.failed > 0) {
    throw new Error(`[demonstrator:site] map tile prefetch failed for ${result.failed} tile(s)`);
  }
}
