#!/usr/bin/env bun
/** 🧭 `@gis/map/play` task router: `bun ./script.ts <dev|build|test|tiles>`. */
import { join } from "node:path";
import {
  GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
  GIS_MAP_PREFETCH_RASTER_Z_MAX,
  GIS_MAP_VECTOR_TILE_MAX_Z,
  prefetchMapTiles,
  type GisMapPrefetchBounds,
} from "../../../ui/styling/vite-elements-assets.ts";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runViteBunxDev,
  runVitest,
} from "../../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runViteBunxDev(this.root, segments, { portEnv: "GIS_MAP_PLAY_PORT", defaultPort: "6040" });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "gis_map"], this.repoRoot, playPollingEnv());
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
  }
}

function parseFlagInt(segments: string[], name: string): number | undefined {
  const prefix = `--${name}=`;
  const hit = segments.find((s) => s.startsWith(prefix));
  if (!hit) {
    return undefined;
  }
  const n = Number(hit.slice(prefix.length));
  return Number.isFinite(n) ? Math.floor(n) : undefined;
}

function parsePrefetchBounds(segments: string[]): GisMapPrefetchBounds {
  const num = (key: string, fallback: number) => parseFlagInt(segments, key) ?? fallback;
  return {
    west: num("west", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.west),
    south: num("south", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.south),
    east: num("east", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.east),
    north: num("north", GIS_MAP_DEFAULT_PREFETCH_BOUNDS.north),
  };
}

class TilesScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const rasterOnly = segments.includes("--raster-only");
    const vectorOnly = segments.includes("--vector-only");
    const fullRaster = segments.includes("--full-raster");
    const zMinRaster = parseFlagInt(segments, "raster-z-min") ?? 0;
    const zMaxRaster = parseFlagInt(segments, "raster-z-max") ?? GIS_MAP_PREFETCH_RASTER_Z_MAX;
    const zMinVector = parseFlagInt(segments, "vector-z-min") ?? 0;
    const zMaxVector = parseFlagInt(segments, "vector-z-max") ?? GIS_MAP_VECTOR_TILE_MAX_Z;
    const concurrency = parseFlagInt(segments, "concurrency") ?? 4;
    await prefetchMapTiles({
      repoRoot: this.repoRoot,
      bounds: parsePrefetchBounds(segments),
      raster: !vectorOnly,
      vector: !rasterOnly,
      zMinRaster,
      zMaxRaster,
      zMinVector,
      zMaxVector,
      concurrency,
    });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("tiles", TilesScript);

await runBundleScriptMain(router, import.meta.url);
