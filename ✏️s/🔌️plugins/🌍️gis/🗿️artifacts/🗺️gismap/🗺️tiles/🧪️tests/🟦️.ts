import assert from "node:assert/strict";
import Ajv from "ajv";
import { geoMercator } from "d3-geo";
import { createServer } from "node:http";
import { mkdtempSync, mkdirSync, existsSync, writeFileSync, readFileSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { once } from "node:events";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { GIS_MAP_DEFAULT_PREFETCH_BOUNDS, listMapTilesForBounds, lonLatToTileXY, mapTileCacheRoots, prefetchMapTiles, type GisMapTileProvidersV1 } from "../🟦️.ts";

/** 🧪️ Checks schema-owned coordinates and real bounded HTTP cache work against independent oracles. */
export async function proveGisMapTilesV1(artifactRoot: string): Promise<number> {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  const projection = geoMercator().scale(1 / (2 * Math.PI)).translate([0.5, 0.5]);
  for (const row of corpus.coordinates) {
    const actual = lonLatToTileXY(row.longitude, row.latitude, row.zoom);
    const point = projection([row.longitude, row.latitude])!;
    const n = 2 ** row.zoom;
    const oracle = { x: Math.max(0, Math.min(n - 1, Math.floor(point[0] * n))), y: Math.max(0, Math.min(n - 1, Math.floor(point[1] * n))) };
    assert.deepEqual(oracle, row.expected);
    assert.deepEqual(actual, JSON.parse(JSON.stringify(oracle)));
  }
  for (const row of corpus.plans) {
    assert(validate(row.input), JSON.stringify(validate.errors));
    const tiles = listMapTilesForBounds(row.input.bounds, row.input.zMin, row.input.zMax);
    assert.equal(tiles.length, row.count);
    assert.deepEqual(tiles[0], row.first);
    assert.deepEqual(tiles.at(-1), row.last);
  }
  assert(listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 8, 8).length > listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 2, 2).length);
  for (const row of corpus.hostile) assert.throws(() => listMapTilesForBounds(row.bounds, row.zMin, row.zMax));
  mkdirSync(artifactRoot, { recursive: true });
  const sandbox = mkdtempSync(join(artifactRoot, "gis-tiles-"));
  let requests = 0;
  let delay = false;
  let oversized = false;
  let base = "";
  const abort = new AbortController();
  const server = createServer((request, response) => {
    requests++;
    assert.equal(request.headers["user-agent"], "OwnedTileOracle/1");
    if (delay) { abort.abort(); return; }
    if (oversized) { response.setHeader("content-length", 3 * 1024 * 1024); response.end(); return; }
    if (request.url === "/vector.json") { response.setHeader("content-type", "application/json"); response.end(JSON.stringify({ tiles: [base + "/vector/{z}/{x}/{y}.pbf"] })); }
    else response.end(Buffer.from([1, 2, 3, 4]));
  });
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const address = server.address();
  assert(address && typeof address === "object");
  base = "http://127.0.0.1:" + address.port;
  const providers: GisMapTileProvidersV1 = { rasterTemplate: base + "/raster/{z}/{x}/{y}.png", vectorTileJson: base + "/vector.json", userAgent: "OwnedTileOracle/1" };
  try {
    const progress: [number, number][] = [];
    const initial = await prefetchMapTiles({ repoRoot: sandbox, zMaxRaster: 0, zMaxVector: 0, delayMs: 0, providers, progress: (completed, total) => progress.push([completed, total]), log: () => {} });
    assert.deepEqual(initial, { downloaded: 2, skipped: 0, failed: 0, cancelled: false });
    assert.equal(requests, 3);
    assert.deepEqual(progress.at(-1), [2, 2]);
    const roots = mapTileCacheRoots(sandbox);
    assert.deepEqual([...readFileSync(resolve(roots.osm, "0/0/0.png"))], [1, 2, 3, 4]);
    const skipped = await prefetchMapTiles({ repoRoot: sandbox, zMaxRaster: 0, zMaxVector: 0, delayMs: 0, providers, log: () => {} });
    assert.deepEqual(skipped, { downloaded: 0, skipped: 2, failed: 0, cancelled: false });
    assert.equal(requests, 3);
    oversized = true;
    const refused = await prefetchMapTiles({ repoRoot: sandbox, vector: false, zMaxRaster: 0, skipExisting: false, providers, log: () => {} });
    assert.deepEqual(refused, { downloaded: 0, skipped: 0, failed: 1, cancelled: false });
    assert.deepEqual([...readFileSync(resolve(roots.osm, "0/0/0.png"))], [1, 2, 3, 4]);
    oversized = false;
    delay = true;
    const cancelled = await prefetchMapTiles({ repoRoot: join(sandbox, "cancelled"), vector: false, zMaxRaster: 0, delayMs: 0, providers, signal: abort.signal, log: () => {} });
    assert.equal(cancelled.cancelled, true);
    assert.equal(cancelled.downloaded, 0);
    assert(!existsSync(resolve(mapTileCacheRoots(join(sandbox, "cancelled")).osm, "0/0/0.png")));
    return corpus.coordinates.length + corpus.plans.length + corpus.hostile.length + 4;
  } finally {
    await new Promise<void>((resolveClose, reject) => { server.close(error => error ? reject(error) : resolveClose()); server.closeAllConnections(); });
    rmSync(sandbox, { recursive: true, force: true });
  }
}
