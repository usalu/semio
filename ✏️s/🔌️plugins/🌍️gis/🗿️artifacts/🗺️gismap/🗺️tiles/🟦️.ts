import planSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { ephemeralMap } from "@semio-tech/framework";
import { existsSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { isAbsolute, relative, resolve } from "node:path";

//#region 🔖️MapTileCache
/** 🗺️ Compliant User-Agent for OSM / MapLibre demotiles in map play. */
export const GIS_MAP_TILE_USER_AGENT = "ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";

/** 🗺️ Default dev prefetch bounds (Switzerland) for GIS map play. */
export const GIS_MAP_DEFAULT_PREFETCH_BOUNDS = Object.freeze({ ...planSchema.properties.bounds.default });

export type GisMapPrefetchBounds = {
  readonly west: number;
  readonly south: number;
  readonly east: number;
  readonly north: number;
};

export const GIS_MAP_OSM_TILE_MAX_Z = 19;
/** 🗺️ OpenFreeMap / OpenMapTiles planet MVT (OSM); matches raster detail up to z14. */
export const GIS_MAP_VECTOR_TILE_MAX_Z = 14;
export const GIS_MAP_OPENFREEMAP_TILEJSON = "https://tiles.openfreemap.org/planet";

/** 🌐️ Owner-controlled raster and vector upstream coordinates. */
export type GisMapTileProvidersV1 = { readonly rasterTemplate: string; readonly vectorTileJson: string; readonly userAgent: string };
export const GIS_MAP_TILE_PROVIDERS: GisMapTileProvidersV1 = { rasterTemplate: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", vectorTileJson: GIS_MAP_OPENFREEMAP_TILEJSON, userAgent: GIS_MAP_TILE_USER_AGENT };

/** 🗺️ Highest zoom prefetched for offline map play (matches `GIS_MAP_LOD_TILE_Z` building band). */
export const GIS_MAP_PREFETCH_RASTER_Z_MAX = 13;

export function mapTileCacheRoots(repoRoot: string): { readonly osm: string; readonly vt: string } {
  return {
    osm: resolve(repoRoot, ".🧬semio/🗺️map", "osm-tiles"),
    vt: resolve(repoRoot, ".🧬semio/🗺️map", "openfreemap-vt"),
  };
}

/** 🧭️ Web Mercator tile index for a lon/lat at zoom `z`. */
export function lonLatToTileXY(lon: number, lat: number, z: number): { x: number; y: number } {
  if (!Number.isInteger(z) || z < 0 || z > planSchema.properties.zMax.maximum || !Number.isFinite(lon) || lon < -180 || lon > 180 || !Number.isFinite(lat) || Math.abs(lat) > planSchema.properties.bounds.properties.north.maximum) throw Error("invalid map tile coordinate");
  const n = 2 ** z;
  const x = Math.floor(((lon + 180) / 360) * n);
  const latRad = (lat * Math.PI) / 180;
  const y = Math.floor(((1 - Math.log(Math.tan(latRad) + 1 / Math.cos(latRad)) / Math.PI) / 2) * n);
  return { x: Math.max(0, Math.min(n - 1, x)), y: Math.max(0, Math.min(n - 1, y)) };
}

/** 📐️ Inclusive OSM tile index range covering `bounds` at zoom `z`. */
export function tileRangeForBounds(bounds: GisMapPrefetchBounds, z: number): { x0: number; x1: number; y0: number; y1: number } {
  const sw = lonLatToTileXY(bounds.west, bounds.south, z);
  const ne = lonLatToTileXY(bounds.east, bounds.north, z);
  return {
    x0: Math.min(sw.x, ne.x),
    x1: Math.max(sw.x, ne.x),
    y0: Math.min(sw.y, ne.y),
    y1: Math.max(sw.y, ne.y),
  };
}

export type GisMapTileCoord = { readonly z: number; readonly x: number; readonly y: number };

/** 📋️ Lists every tile in `bounds` for zoom levels `zMin`…`zMax` (inclusive). */
export function listMapTilesForBounds(bounds: GisMapPrefetchBounds, zMin: number, zMax: number, maxTiles = planSchema.properties.maxTiles.default): GisMapTileCoord[] {
  if (!Number.isInteger(zMin) || !Number.isInteger(zMax) || zMin < 0 || zMax < zMin || zMax > planSchema.properties.zMax.maximum || !Number.isInteger(maxTiles) || maxTiles < 1 || maxTiles > planSchema.properties.maxTiles.maximum || bounds.west > bounds.east || bounds.south > bounds.north) throw Error("invalid map tile plan");
  const lo = zMin;
  const hi = zMax;
  const out: GisMapTileCoord[] = [];
  for (let z = lo; z <= hi; z++) {
    const { x0, x1, y0, y1 } = tileRangeForBounds(bounds, z);
    if (out.length + (x1 - x0 + 1) * (y1 - y0 + 1) > maxTiles) throw Error("map tile plan exceeds its job budget");
    for (let x = x0; x <= x1; x++) {
      for (let y = y0; y <= y1; y++) {
        out.push({ z, x, y });
      }
    }
  }
  return out;
}

export type PrefetchMapTilesResult = {
  readonly downloaded: number;
  readonly skipped: number;
  readonly failed: number;
  readonly cancelled: boolean;
};

export type PrefetchMapTilesOptions = {
  readonly repoRoot: string;
  readonly bounds?: GisMapPrefetchBounds;
  readonly raster?: boolean;
  readonly vector?: boolean;
  readonly zMinRaster?: number;
  readonly zMaxRaster?: number;
  readonly zMinVector?: number;
  readonly zMaxVector?: number;
  readonly concurrency?: number;
  readonly skipExisting?: boolean;
  readonly delayMs?: number;
  readonly log?: (line: string) => void;
  readonly signal?: AbortSignal;
  readonly providers?: GisMapTileProvidersV1;
  readonly progress?: (completed: number, total: number) => void;
};

async function boundedTileBytes(response: Response, maximum: number): Promise<Uint8Array> {
  if (Number(response.headers.get("content-length")) > maximum) throw Error("map tile body exceeds its byte budget");
  if (!response.body) return new Uint8Array();
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let length = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      length += value.byteLength;
      if (length > maximum) throw Error("map tile body exceeds its byte budget");
      chunks.push(value);
    }
  } finally { await reader.cancel(); }
  const output = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { output.set(chunk, offset); offset += chunk.byteLength; }
  return output;
}

async function fetchOsmTileToCache(cacheRoot: string, z: number, x: number, y: number, signal?: AbortSignal, providers = GIS_MAP_TILE_PROVIDERS): Promise<boolean> {
  const rel = `${z}/${x}/${y}.png`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const upstream = await fetch(providers.rasterTemplate.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y)), {
    headers: { "User-Agent": providers.userAgent }, signal: AbortSignal.any([AbortSignal.timeout(planSchema.definitions.TilePlanLimits.default.requestMs), ...(signal ? [signal] : [])]),
  });
  if (!upstream.ok) {
    return false;
  }
  const bytes = await boundedTileBytes(upstream, planSchema.definitions.TilePlanLimits.default.tileBytes);
  if (signal?.aborted) return false;
  await writeFile(filePath, bytes);
  return true;
}

const vectorTileTemplates = ephemeralMap<string, { template: string; at: number }>("gis.gismap.tiles.vector-templates");
const OPENFREEMAP_TILE_TEMPLATE_TTL_MS = planSchema.definitions.TilePlanLimits.default.templateTtlMs;

async function resolveOpenFreeMapTileTemplate(signal?: AbortSignal, providers = GIS_MAP_TILE_PROVIDERS): Promise<string> {
  const now = Date.now();
  const key = JSON.stringify([providers.vectorTileJson, providers.userAgent]);
  const cached = vectorTileTemplates.get(key);
  if (cached && now - cached.at < OPENFREEMAP_TILE_TEMPLATE_TTL_MS) return cached.template;
  const res = await fetch(providers.vectorTileJson, { headers: { "User-Agent": providers.userAgent }, signal: AbortSignal.any([AbortSignal.timeout(planSchema.definitions.TilePlanLimits.default.requestMs), ...(signal ? [signal] : [])]) });
  if (!res.ok) {
    throw new Error(`OpenFreeMap TileJSON failed: ${res.status}`);
  }
  const json = JSON.parse(new TextDecoder().decode(await boundedTileBytes(res, planSchema.definitions.TilePlanLimits.default.templateBytes))) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("OpenFreeMap TileJSON missing tiles URL template");
  }
  vectorTileTemplates.set(key, { template, at: now });
  return template;
}

async function fetchVtTileToCache(cacheRoot: string, z: number, x: number, y: number, signal?: AbortSignal, providers = GIS_MAP_TILE_PROVIDERS): Promise<boolean> {
  const rel = `${z}/${x}/${y}.pbf`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const template = await resolveOpenFreeMapTileTemplate(signal, providers);
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstream = await fetch(url, { headers: { "User-Agent": providers.userAgent }, signal: AbortSignal.any([AbortSignal.timeout(planSchema.definitions.TilePlanLimits.default.requestMs), ...(signal ? [signal] : [])]) });
  if (!upstream.ok) {
    return false;
  }
  const buf = await boundedTileBytes(upstream, planSchema.definitions.TilePlanLimits.default.tileBytes);
  if (buf.length === 0) {
    return false;
  }
  if (signal?.aborted) return false;
  await writeFile(filePath, buf);
  return true;
}

/** ⬇️ Prefetch OSM PNG and MapLibre MVT tiles into `.🧬semio/🗺️map` for offline map play. */
export async function prefetchMapTiles(options: PrefetchMapTilesOptions): Promise<PrefetchMapTilesResult> {
  const {
    repoRoot,
    bounds = GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
    raster = true,
    vector = true,
    zMinRaster = 0,
    zMaxRaster = GIS_MAP_PREFETCH_RASTER_Z_MAX,
    zMinVector = 0,
    zMaxVector = GIS_MAP_VECTOR_TILE_MAX_Z,
    concurrency = 4,
    skipExisting = true,
    delayMs = 120,
    log = (line) => console.log(line),
  } = options;
  if (!Number.isInteger(concurrency) || concurrency < 1 || concurrency > planSchema.definitions.TilePlanLimits.default.concurrency || !Number.isInteger(delayMs) || delayMs < 0 || delayMs > planSchema.definitions.TilePlanLimits.default.delayMs) throw Error("invalid map prefetch scheduling");
  if (options.signal?.aborted) return { downloaded: 0, skipped: 0, failed: 0, cancelled: true };
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const jobs: { kind: "osm" | "vt"; z: number; x: number; y: number }[] = [];
  if (raster) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinRaster, Math.min(zMaxRaster, GIS_MAP_OSM_TILE_MAX_Z))) {
      jobs.push({ kind: "osm", z, x, y });
    }
  }
  if (vector) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinVector, Math.min(zMaxVector, GIS_MAP_VECTOR_TILE_MAX_Z))) {
      jobs.push({ kind: "vt", z, x, y });
    }
  }
  if (jobs.length > planSchema.properties.maxTiles.maximum) throw Error("map prefetch exceeds its job budget");
  const zoomLabel = `(raster z${zMinRaster}-${zMaxRaster}, vector z${zMinVector}-${zMaxVector})`;
  let skipped = 0;
  const pending = skipExisting
    ? jobs.filter((job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ext = job.kind === "osm" ? "png" : "pbf";
        const filePath = resolve(cacheRoot, `${job.z}/${job.x}/${job.y}.${ext}`);
        if (existsSync(filePath)) {
          skipped++;
          return false;
        }
        return true;
      })
    : jobs;
  log(`[gis/2d/play] prefetch ${jobs.length} tiles ${zoomLabel}` + (skipExisting ? ` (${skipped} cached, ${pending.length} to fetch)` : ""));
  options.progress?.(skipped, jobs.length);
  if (pending.length === 0) {
    log(`[gis/2d/play] prefetch done: downloaded=0 skipped=${skipped} failed=0`);
    return { downloaded: 0, skipped, failed: 0, cancelled: options.signal?.aborted === true };
  }
  let downloaded = 0;
  let failed = 0;
  const sleep = (ms: number) => new Promise<void>(resolveDelay => {
    const done = () => { clearTimeout(timer); options.signal?.removeEventListener("abort", done); resolveDelay(); };
    const timer = setTimeout(done, ms);
    options.signal?.addEventListener("abort", done, { once: true });
    if (options.signal?.aborted) done();
  });
  for (let i = 0; i < pending.length; i += concurrency) {
    if (options.signal?.aborted) break;
    const batch = pending.slice(i, i + concurrency);
    await Promise.all(
      batch.map(async (job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ok = await (job.kind === "osm" ? fetchOsmTileToCache(cacheRoot, job.z, job.x, job.y, options.signal, options.providers) : fetchVtTileToCache(cacheRoot, job.z, job.x, job.y, options.signal, options.providers)).catch(() => false);
        if (options.signal?.aborted) return;
        if (ok) {
          downloaded++;
        } else {
          failed++;
        }
        options.progress?.(downloaded + failed + skipped, jobs.length);
      }),
    );
    if (delayMs > 0 && i + concurrency < pending.length) {
      await sleep(delayMs);
    }
  }
  log(`[gis/2d/play] prefetch done: downloaded=${downloaded} skipped=${skipped} failed=${failed}`);
  return { downloaded, skipped, failed, cancelled: options.signal?.aborted === true };
}
//#endregion 🔖️MapTileCache
