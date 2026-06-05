// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `ui/assets` at `/assets/*` (fonts, cursors, …). */
// #endregion 🧲Header

// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { cpSync, createReadStream, existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { isAbsolute, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Connect, Plugin } from "vite";
import { defineConfig, type UserConfig } from "vite";
import { PLAYGROUND_SITE_DEV_PORTS, PLAYGROUND_SITE_HOSTS, playgroundEmbedUrl, type PlaygroundSiteKind } from "./playground-embed-url.ts";
// #endregion 🔌Adapters

export { PLAYGROUND_SITE_DEV_PORTS, PLAYGROUND_SITE_HOSTS, playgroundEmbedUrl, type PlaygroundSiteKind };

//#region 🔖ViteElementsAssets
/** @emoji 📦 Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
export function playgroundStaticSiteBuildOptions(overrides?: UserConfig["build"]): NonNullable<UserConfig["build"]> {
  return {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    ...overrides,
  };
}

/** @emoji 🔗 True when a request targets Vite prebundled `node_modules/.vite/deps` chunks. */
export function isPlaygroundOptimizedDepUrl(url: string): boolean {
  return url.includes("/node_modules/.vite/deps/");
}

/** @emoji 🔄 Full-reload connected clients when a stale optimized-dep chunk returns 504. */
export function playgroundStaleOptimizeDepPlugin(): Plugin {
  return {
    name: "playground-stale-optimize-dep",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!isPlaygroundOptimizedDepUrl(req.url ?? "")) {
          next();
          return;
        }
        res.on("finish", () => {
          if (res.statusCode === 504) {
            server.ws.send({ type: "full-reload", path: "*" });
          }
        });
        next();
      });
    },
  };
}

/** @emoji 🖼 Dev/preview CSP so playgrounds can be iframe-embedded locally. */
export function playgroundIframeEmbedHeadersPlugin(): Plugin {
  const useHeaders: Connect.NextHandleFunction = (_req, res, next) => {
    res.setHeader("Content-Security-Policy", "frame-ancestors *");
    next();
  };
  return {
    name: "playground-iframe-embed-headers",
    configureServer(server) {
      server.middlewares.use(useHeaders);
    },
    configurePreviewServer(server) {
      server.middlewares.use(useHeaders);
    },
  };
}

function contentTypeForUiAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".woff2")) {
    return "font/woff2";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  return undefined;
}

function createUiAssetsMiddleware(assetsRoot: string): Connect.NextHandleFunction {
  const assetsRootResolved = resolve(assetsRoot);
  return (req, res, next) => {
    if (!req.url?.startsWith("/assets/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/assets/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(assetsRootResolved, rel);
    const relToRoot = relative(assetsRootResolved, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    const contentType = contentTypeForUiAsset(filePath);
    if (contentType) {
      res.setHeader("Content-Type", contentType);
    }
    createReadStream(filePath).pipe(res);
  };
}

/** @emoji 📂 Kit fixture GLB roots for puzzle 3d `/meshes/*` URLs. */
export function puzzle3dKitMeshRoots(repoRoot: string): { readonly meshRoots: readonly string[]; readonly placeholderMesh: string } {
  return {
    meshRoots: [
      resolve(repoRoot, "semio/fixtures/kit/dev/metabolism/representations"),
      resolve(repoRoot, "semio/fixtures/kit/folder/abbau-aufbau"),
    ],
    placeholderMesh: resolve(repoRoot, "semio/assets/mesh/placeholder.glb"),
  };
}

/** @emoji 🌐 Connect middleware: serve kit GLBs at `/meshes/<name>.glb` (first matching root wins). */
export function createPuzzle3dMeshesMiddleware(meshRoots: readonly string[], placeholderMesh: string): Connect.NextHandleFunction {
  const rootsResolved = meshRoots.map((root) => resolve(root));
  const placeholderResolved = resolve(placeholderMesh);
  return (req, res, next) => {
    if (!req.url?.startsWith("/meshes/")) {
      next();
      return;
    }
    const rawName = decodeURIComponent(req.url.slice("/meshes/".length).split(/[?#]/, 1)[0] ?? "");
    if (rawName === "placeholder.glb") {
      if (!existsSync(placeholderResolved) || !statSync(placeholderResolved).isFile()) {
        next();
        return;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(placeholderResolved).pipe(res);
      return;
    }
    for (const meshRootResolved of rootsResolved) {
      const filePath = resolve(meshRootResolved, rawName);
      const relToRoot = relative(meshRootResolved, filePath);
      if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
        continue;
      }
      if (!existsSync(filePath) || !statSync(filePath).isFile()) {
        continue;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(filePath).pipe(res);
      return;
    }
    next();
  };
}

/** @emoji 🧊 Vite: serve and copy kit meshes at `/meshes/*` for puzzle 3d play and sketchpad. */
export function puzzle3dMeshesVitePlugin(repoRoot: string): Plugin[] {
  const { meshRoots, placeholderMesh } = puzzle3dKitMeshRoots(repoRoot);
  const serveMeshes = createPuzzle3dMeshesMiddleware(meshRoots, placeholderMesh);
  let viteRoot = process.cwd();
  return [
    {
      name: "puzzle-3d-meshes-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveMeshes);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveMeshes);
      },
    },
    {
      name: "puzzle-3d-meshes-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dest = resolve(viteRoot, "dist", "meshes");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        for (const meshRoot of meshRoots) {
          if (!existsSync(meshRoot)) {
            continue;
          }
          cpSync(meshRoot, dest, { recursive: true });
        }
        if (existsSync(placeholderMesh)) {
          cpSync(placeholderMesh, resolve(dest, "placeholder.glb"));
        }
      },
    },
  ];
}

/** @emoji 🌐 Vite: serve and copy `ui/assets` at `/assets/*` for palette fonts and cursors. */
export function uiAssetsVitePlugin(assetsRoot: string): Plugin[] {
  let viteRoot = process.cwd();
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  return [
    {
      name: "ui-assets-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
    },
    {
      name: "ui-assets-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(assetsRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "assets");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🛝 Shared Vite preset for puzzle play harnesses (assets, renderer subpaths, workspace aliases). */
const PLAYGROUND_RENDERER_SHELL_SUBPATHS = ["@framework/playground/renderer/react/shell", "@framework/playground/renderer/react/boot"] as const;

const PLAYGROUND_RENDERER_PUZZLE_HOSTS_START = "//#region 🔖Puzzle3dPlayHost";
const PLAYGROUND_RENDERER_BOOT_START = "//#region 🔖Boot";
const PLAYGROUND_RENDERER_VITEST_START = "//#region 🧪Tests";

export type PlaygroundRendererPuzzleKind = "2d" | "3d" | "5d" | "map" | "presentation" | "wires";

const PLAYGROUND_RENDERER_PUZZLE_BOOT_SUBPATHS: Readonly<Record<string, PlaygroundRendererPuzzleKind>> = {
  "@framework/playground/renderer/react/puzzle/2d": "2d",
  "@framework/playground/renderer/react/puzzle/3d": "3d",
  "@framework/playground/renderer/react/puzzle/5d": "5d",
  "@framework/playground/renderer/react/puzzle/map": "map",
  "@framework/playground/renderer/react/presentation": "presentation",
  "@framework/playground/renderer/react/reasoning/wires": "wires",
};

const PLAYGROUND_RENDERER_PUZZLE_HOST_MARKERS: Readonly<Record<PlaygroundRendererPuzzleKind, { readonly start: string; readonly end: string }>> = {
  "3d": { start: "//#region 🔖Puzzle3dPlayHost", end: "//#endregion 🔖Puzzle3dPlayHost" },
  "5d": { start: "//#region 🔖Puzzle5dPlayHost", end: "//#endregion 🔖Puzzle5dPlayHost" },
  "2d": { start: "//#region 🔖Puzzle2dPlayHost", end: "//#endregion 🔖Puzzle2dPlayHost" },
  map: { start: "//#region 🔖MapPlayHost", end: "//#endregion 🔖MapPlayHost" },
  presentation: { start: "//#region 🔖PresentationPlayHost", end: "//#endregion 🔖PresentationPlayHost" },
  wires: { start: "//#region 🔖Puzzle2dPlayHost", end: "//#endregion 🔖Puzzle2dPlayHost" },
};

function slicePlaygroundRendererRegion(source: string, startMarker: string, endMarker: string): string {
  const start = source.indexOf(startMarker);
  if (start < 0) return "";
  const end = source.indexOf(endMarker, start);
  if (end < 0) return "";
  return source.slice(start, end + endMarker.length);
}

/** @emoji ✂️ Drops puzzle play hosts from monolithic renderer `index.tsx` (shell + boot + optional vitest). */
export function stripPlaygroundRendererPuzzleHosts(source: string, options: { readonly includeVitest?: boolean } = {}): string {
  const puzzleStart = source.indexOf(PLAYGROUND_RENDERER_PUZZLE_HOSTS_START);
  const bootStart = source.indexOf(PLAYGROUND_RENDERER_BOOT_START);
  const testsStart = source.indexOf(PLAYGROUND_RENDERER_VITEST_START);
  if (puzzleStart < 0 || bootStart < 0) return source;
  const bootEnd = testsStart >= 0 ? testsStart : source.length;
  let out = `${source.slice(0, puzzleStart)}${source.slice(bootStart, bootEnd)}`;
  if (options.includeVitest && testsStart >= 0) out += source.slice(testsStart);
  return out;
}

/** @emoji ✂️ Keeps shell + one puzzle play host + boot (per-dimension playground entries). */
export function stripPlaygroundRendererForPuzzleKind(
  source: string,
  kind: PlaygroundRendererPuzzleKind,
  options: { readonly includeVitest?: boolean } = {},
): string {
  const puzzleStart = source.indexOf(PLAYGROUND_RENDERER_PUZZLE_HOSTS_START);
  const bootStart = source.indexOf(PLAYGROUND_RENDERER_BOOT_START);
  const testsStart = source.indexOf(PLAYGROUND_RENDERER_VITEST_START);
  const markers = PLAYGROUND_RENDERER_PUZZLE_HOST_MARKERS[kind];
  if (puzzleStart < 0 || bootStart < 0) return source;
  const bootEnd = testsStart >= 0 ? testsStart : source.length;
  const host = slicePlaygroundRendererRegion(source, markers.start, markers.end);
  let out = `${source.slice(0, puzzleStart)}${host}${source.slice(bootStart, bootEnd)}`;
  if (options.includeVitest && testsStart >= 0) out += source.slice(testsStart);
  return out;
}

function namedImportSpecifiersForModule(source: string, moduleId: string): string[] {
  const escaped = moduleId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`import\\s*\\{([^}]+)\\}\\s*from\\s*["']${escaped}["']`, "gs");
  const names: string[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    for (const part of match[1].split(",")) {
      const trimmed = part.trim();
      if (!trimmed) continue;
      const name = trimmed.replace(/^type\s+/, "").split(/\s+as\s+/)[0]?.trim();
      if (name) names.push(name);
    }
  }
  return names;
}

/** @emoji 🔁 Named import specifiers duplicated within the same module import block(s). */
export function duplicateNamedImportsForModule(source: string, moduleId: string): string[] {
  const names = namedImportSpecifiersForModule(source, moduleId);
  const seen = new Set<string>();
  const dupes: string[] = [];
  for (const name of names) {
    if (seen.has(name)) dupes.push(name);
    else seen.add(name);
  }
  return dupes;
}

/** @emoji 🛝 Vite virtual entry for shell/boot and per-puzzle boot subpaths without cross-dimension hosts. */
export function playgroundRendererShellEntryPlugin(rendererIndexPath: string): Plugin {
  return {
    name: "playground-renderer-shell-entry",
    enforce: "pre",
    resolveId(source) {
      if ((PLAYGROUND_RENDERER_SHELL_SUBPATHS as readonly string[]).includes(source)) {
        return `${rendererIndexPath}?playgroundEntry=shell`;
      }
      const kind = PLAYGROUND_RENDERER_PUZZLE_BOOT_SUBPATHS[source];
      if (kind) return `${rendererIndexPath}?playgroundEntry=puzzle-${kind}`;
    },
    load(id) {
      if (!id.startsWith(rendererIndexPath) || !id.includes("playgroundEntry=")) return;
      const source = readFileSync(rendererIndexPath, "utf8");
      if (id.includes("playgroundEntry=shell")) {
        return stripPlaygroundRendererPuzzleHosts(source, { includeVitest: false });
      }
      const puzzleMatch = id.match(/playgroundEntry=puzzle-(2d|3d|5d|map|presentation|wires)/);
      if (puzzleMatch) {
        return stripPlaygroundRendererForPuzzleKind(source, puzzleMatch[1] as PlaygroundRendererPuzzleKind, { includeVitest: false });
      }
    },
  };
}

/** @emoji 🧪 Vitest load: shell-only slice of renderer `index.tsx` when `PLAYGROUND_RENDERER_SHELL_ONLY=1`. */
export function playgroundRendererVitestShellOnlyPlugin(rendererIndexPath: string): Plugin {
  return {
    name: "playground-renderer-vitest-shell-only",
    enforce: "pre",
    load(id) {
      if (process.env.PLAYGROUND_RENDERER_SHELL_ONLY !== "1") return;
      const filePath = id.split("?")[0];
      if (filePath !== rendererIndexPath) return;
      return stripPlaygroundRendererPuzzleHosts(readFileSync(rendererIndexPath, "utf8"), { includeVitest: true });
    },
  };
}

export type PlaygroundPlayViteOptions = {
  readonly playDir: string;
  readonly repoRoot: string;
  /** @emoji 🎯 When set, `import.meta.env.PUZZLE_PLAY_ENTRY` gates browser boot in that play's `index.ts`. */
  readonly playEntryKind?: PlaygroundRendererPuzzleKind;
  readonly extraAliases?: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }>;
  readonly extraPlugins?: readonly Plugin[];
  readonly watchIgnored?: readonly string[];
  readonly build?: UserConfig["build"];
  readonly server?: UserConfig["server"];
  readonly optimizeDeps?: UserConfig["optimizeDeps"];
  readonly resolveDedupe?: readonly string[];
};

/** @emoji 🎬 R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
export const PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"] as const;

//#region 🔖MapTileCache
/** @emoji 🗺 Compliant User-Agent for OSM / MapLibre demotiles in map play. */
export const GIS_MAP_TILE_USER_AGENT = "SemioGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";

/** @emoji 🗺 Default dev prefetch bounds (Switzerland) for GIS map play. */
export const GIS_MAP_DEFAULT_PREFETCH_BOUNDS = {
  west: 5.95,
  south: 45.82,
  east: 10.52,
  north: 47.81,
} as const;

export type GisMapPrefetchBounds = {
  readonly west: number;
  readonly south: number;
  readonly east: number;
  readonly north: number;
};

export const GIS_MAP_OSM_TILE_MAX_Z = 19;
/** @emoji 🗺 OpenFreeMap / OpenMapTiles planet MVT (OSM); matches raster detail up to z14. */
export const GIS_MAP_VECTOR_TILE_MAX_Z = 14;
export const GIS_MAP_OPENFREEMAP_TILEJSON = "https://tiles.openfreemap.org/planet";
/** @emoji 🗺 Highest zoom prefetched for offline map play (matches `GIS_MAP_LOD_TILE_Z` building band). */
export const GIS_MAP_PREFETCH_RASTER_Z_MAX = 13;

export function mapTileCacheRoots(repoRoot: string): { readonly osm: string; readonly vt: string } {
  return {
    osm: resolve(repoRoot, ".repo-cache", "osm-tiles"),
    vt: resolve(repoRoot, ".repo-cache", "openfreemap-vt"),
  };
}

/** @emoji 🧭 Web Mercator tile index for a lon/lat at zoom `z`. */
export function lonLatToTileXY(lon: number, lat: number, z: number): { x: number; y: number } {
  const n = 2 ** z;
  const x = Math.floor(((lon + 180) / 360) * n);
  const latRad = (lat * Math.PI) / 180;
  const y = Math.floor(((1 - Math.log(Math.tan(latRad) + 1 / Math.cos(latRad)) / Math.PI) / 2) * n);
  return { x: Math.max(0, Math.min(n - 1, x)), y: Math.max(0, Math.min(n - 1, y)) };
}

/** @emoji 📐 Inclusive OSM tile index range covering `bounds` at zoom `z`. */
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

/** @emoji 📋 Lists every tile in `bounds` for zoom levels `zMin`…`zMax` (inclusive). */
export function listMapTilesForBounds(bounds: GisMapPrefetchBounds, zMin: number, zMax: number): GisMapTileCoord[] {
  const lo = Math.max(0, Math.min(zMin, zMax));
  const hi = Math.max(lo, zMax);
  const out: GisMapTileCoord[] = [];
  for (let z = lo; z <= hi; z++) {
    const { x0, x1, y0, y1 } = tileRangeForBounds(bounds, z);
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
};

async function fetchOsmTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.png`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const upstream = await fetch(`https://tile.openstreetmap.org/${z}/${x}/${y}.png`, {
    headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT },
  });
  if (!upstream.ok) {
    return false;
  }
  await writeFile(filePath, Buffer.from(await upstream.arrayBuffer()));
  return true;
}

let openFreeMapTileTemplate: string | null = null;
let openFreeMapTileTemplateAt = 0;
const OPENFREEMAP_TILE_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;

async function resolveOpenFreeMapTileTemplate(): Promise<string> {
  const now = Date.now();
  if (openFreeMapTileTemplate && now - openFreeMapTileTemplateAt < OPENFREEMAP_TILE_TEMPLATE_TTL_MS) {
    return openFreeMapTileTemplate;
  }
  const res = await fetch(GIS_MAP_OPENFREEMAP_TILEJSON, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!res.ok) {
    throw new Error(`OpenFreeMap TileJSON failed: ${res.status}`);
  }
  const json = (await res.json()) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("OpenFreeMap TileJSON missing tiles URL template");
  }
  openFreeMapTileTemplate = template;
  openFreeMapTileTemplateAt = now;
  return template;
}

async function fetchVtTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.pbf`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const template = await resolveOpenFreeMapTileTemplate();
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstream = await fetch(url, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!upstream.ok) {
    return false;
  }
  const buf = Buffer.from(await upstream.arrayBuffer());
  if (buf.length === 0) {
    return false;
  }
  await writeFile(filePath, buf);
  return true;
}

/** @emoji ⬇️ Prefetch OSM PNG and MapLibre MVT tiles into `.repo-cache` for offline map play. */
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
  log(`[gis/map/play] prefetch ${jobs.length} tiles (raster z${zMinRaster}-${zMaxRaster}, vector z${zMinVector}-${zMaxVector})`);
  let downloaded = 0;
  let skipped = 0;
  let failed = 0;
  const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));
  for (let i = 0; i < jobs.length; i += concurrency) {
    const batch = jobs.slice(i, i + concurrency);
    await Promise.all(
      batch.map(async (job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ext = job.kind === "osm" ? "png" : "pbf";
        const filePath = resolve(cacheRoot, `${job.z}/${job.x}/${job.y}.${ext}`);
        if (skipExisting && existsSync(filePath)) {
          skipped++;
          return;
        }
        const ok =
          job.kind === "osm"
            ? await fetchOsmTileToCache(cacheRoot, job.z, job.x, job.y)
            : await fetchVtTileToCache(cacheRoot, job.z, job.x, job.y);
        if (ok) {
          downloaded++;
        } else {
          failed++;
        }
      }),
    );
    if (delayMs > 0 && i + concurrency < jobs.length) {
      await sleep(delayMs);
    }
  }
  log(`[gis/map/play] prefetch done: downloaded=${downloaded} skipped=${skipped} failed=${failed}`);
  return { downloaded, skipped, failed };
}
//#endregion 🔖MapTileCache

/** @emoji 🗺 Dev/preview proxy for OpenStreetMap raster tiles at `/osm/:z/:x/:y.png`. */
export function osmTileProxyVitePlugin(cacheDir: string): Plugin {
  const cacheRoot = resolve(cacheDir, ".repo-cache", "osm-tiles");
  const serveOsm: Connect.NextHandleFunction = async (req, res, next) => {
    const match = req.url?.match(/^\/osm\/(\d+)\/(\d+)\/(\d+)\.png(?:\?.*)?$/);
    if (!match) {
      next();
      return;
    }
    const [, z, x, y] = match;
    const rel = `${z}/${x}/${y}.png`;
    const filePath = resolve(cacheRoot, rel);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      res.setHeader("Content-Type", "image/png");
      createReadStream(filePath).pipe(res);
      return;
    }
    try {
      const ok = await fetchOsmTileToCache(cacheRoot, Number(z), Number(x), Number(y));
      if (!ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader("Content-Type", "image/png");
      createReadStream(filePath).pipe(res);
    } catch {
      res.statusCode = 502;
      res.end();
    }
  };
  return {
    name: "osm-tile-proxy",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use(serveOsm);
    },
    configurePreviewServer(server) {
      server.middlewares.use(serveOsm);
    },
  };
}

/** @emoji 🗺 Dev/preview proxy for OpenFreeMap OSM MVT at `/vt/:z/:x/:y.pbf`. */
export function mapLibreVectorTileProxyVitePlugin(cacheDir: string): Plugin {
  const cacheRoot = resolve(cacheDir, ".repo-cache", "openfreemap-vt");
  const serveVt: Connect.NextHandleFunction = async (req, res, next) => {
    const match = req.url?.match(/^\/vt\/(\d+)\/(\d+)\/(\d+)\.pbf(?:\?.*)?$/);
    if (!match) {
      next();
      return;
    }
    const [, z, x, y] = match;
    const rel = `${z}/${x}/${y}.pbf`;
    const filePath = resolve(cacheRoot, rel);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      res.setHeader("Content-Type", "application/x-protobuf");
      createReadStream(filePath).pipe(res);
      return;
    }
    try {
      const ok = await fetchVtTileToCache(cacheRoot, Number(z), Number(x), Number(y));
      if (!ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader("Content-Type", "application/x-protobuf");
      createReadStream(filePath).pipe(res);
    } catch {
      res.statusCode = 502;
      res.end();
    }
  };
  return {
    name: "maplibre-vt-proxy",
    enforce: "pre",
    configureServer(server) {
      server.middlewares.use(serveVt);
    },
    configurePreviewServer(server) {
      server.middlewares.use(serveVt);
    },
  };
}

/** @emoji 🔀 Vite resolve aliases shared by playground play hosts and renderer vitest graphs. */
export function playgroundRendererResolveAliases(repoRoot: string): ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> {
  const rendererRoot = resolve(repoRoot, "framework/product/playground/renderer/react");
  const rendererIndex = resolve(rendererRoot, "index.tsx");
  return [
    { find: /^@framework\/playground\/renderer\/react$/, replacement: rendererIndex },
    { find: /^@framework\/playground\/core$/, replacement: resolve(repoRoot, "framework/product/playground/core/index.ts") },
    { find: /^@framework\/platform\/core$/, replacement: resolve(repoRoot, "framework/product/platform/core/index.ts") },
    { find: /^@framework\/platform\/renderer\/react$/, replacement: resolve(repoRoot, "framework/product/platform/renderer/react/index.tsx") },
    { find: /^@framework\/core$/, replacement: resolve(repoRoot, "framework/core/index.ts") },
    { find: "@ui/react", replacement: resolve(repoRoot, "ui/react/index.tsx") },
    { find: "@ui/assets", replacement: resolve(repoRoot, "ui/assets/index.ts") },
    { find: "@infinite/cavas/react-renderer", replacement: resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx") },
    { find: "@infinite/world/r3f", replacement: resolve(repoRoot, "infinite/world/r3f/index.tsx") },
    { find: "@puzzle/2d/play", replacement: resolve(repoRoot, "puzzle/2d/play/index.ts") },
    { find: "@puzzle/3d/play", replacement: resolve(repoRoot, "puzzle/3d/play/index.ts") },
    { find: "@puzzle/5d/play", replacement: resolve(repoRoot, "puzzle/5d/play/index.ts") },
    { find: "@puzzle/2d/react", replacement: resolve(repoRoot, "puzzle/2d/react/index.tsx") },
    { find: "@puzzle/3d/react", replacement: resolve(repoRoot, "puzzle/3d/react/index.tsx") },
    { find: "@puzzle/5d/react", replacement: resolve(repoRoot, "puzzle/5d/react/index.tsx") },
    { find: "@gis/map/play", replacement: resolve(repoRoot, "gis/map/play/index.ts") },
    { find: "@gis/map/react", replacement: resolve(repoRoot, "gis/map/react/index.tsx") },
    { find: "@reasoning/mindmap/wires/play", replacement: resolve(repoRoot, "reasoning/mindmap/wires/play/index.ts") },
    { find: "@reasoning/mindmap/wires/react", replacement: resolve(repoRoot, "reasoning/mindmap/wires/react/index.ts") },
    { find: "@reasoning/mindmap/react", replacement: resolve(repoRoot, "reasoning/mindmap/react/index.tsx") },
    { find: "@framework/presentation/play", replacement: resolve(repoRoot, "framework/product/presentation/play/index.ts") },
    { find: "@framework/presentation/core", replacement: resolve(repoRoot, "framework/product/presentation/core/index.ts") },
    { find: "@framework/presentation/renderer/react", replacement: resolve(repoRoot, "framework/product/presentation/renderer/react/index.tsx") },
  ];
}

/** @emoji 🛝 `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
  const { playDir, repoRoot, playEntryKind, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } = options;
  const uiAssetsRoot = resolve(repoRoot, "ui/assets");
  const rendererRoot = resolve(repoRoot, "framework/product/playground/renderer/react");
  const rendererIndex = resolve(rendererRoot, "index.tsx");
  const rendererAliases = playgroundRendererResolveAliases(repoRoot);
  return defineConfig({
    root: playDir,
    base: "./",
    publicDir: resolve(playDir, "public"),
    assetsInclude: ["**/*.wasm"],
    worker: { format: "es" },
    define: playEntryKind ? { "import.meta.env.PUZZLE_PLAY_ENTRY": JSON.stringify(playEntryKind) } : undefined,
    plugins: [
      ...uiAssetsVitePlugin(uiAssetsRoot),
      ...(playEntryKind === "3d" || playEntryKind === "5d" ? puzzle3dMeshesVitePlugin(repoRoot) : []),
      ...(playEntryKind === "map" ? [osmTileProxyVitePlugin(repoRoot), mapLibreVectorTileProxyVitePlugin(repoRoot)] : []),
      tailwindcss(),
      react(),
      playgroundIframeEmbedHeadersPlugin(),
      playgroundStaleOptimizeDepPlugin(),
      playgroundRendererShellEntryPlugin(rendererIndex),
      ...extraPlugins,
    ],
    build: playgroundStaticSiteBuildOptions(build),
    server: {
      fs: { allow: [repoRoot] },
      ...(watchIgnored ? { watch: { ignored: watchIgnored } } : {}),
      ...server,
    },
    resolve: {
      alias: [...rendererAliases, ...extraAliases],
      dedupe: [...PLAYGROUND_SCENE_HOST_DEDUPE, ...(resolveDedupe ?? [])],
    },
    ...(optimizeDeps ? { optimizeDeps } : {}),
  });
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const repoRoot = resolve(fileURLToPath(new URL(".", import.meta.url)), "../..");

  describe("isPlaygroundOptimizedDepUrl", () => {
    it("matches Vite prebundle chunk URLs", () => {
      expect(isPlaygroundOptimizedDepUrl("/node_modules/.vite/deps/chunk-ABC.js?v=1")).toBe(true);
      expect(isPlaygroundOptimizedDepUrl("/index.ts")).toBe(false);
    });
  });

  describe("listMapTilesForBounds", () => {
    it("covers Switzerland at z0 with a single world tile", () => {
      const tiles = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 0, 0);
      expect(tiles).toEqual([{ z: 0, x: 0, y: 0 }]);
    });

    it("returns more tiles at higher zoom", () => {
      const z2 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 2, 2).length;
      const z4 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 4, 4).length;
      expect(z4).toBeGreaterThan(z2);
    });
  });

  describe("puzzle3dKitMeshRoots", () => {
    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      const { meshRoots, placeholderMesh } = puzzle3dKitMeshRoots(repoRoot);
      expect(existsSync(resolve(meshRoots[0]!, "capsule_J.glb"))).toBe(true);
      expect(existsSync(resolve(meshRoots[1]!, "hexagonal-cut-concrete-forest-left.glb"))).toBe(true);
      expect(existsSync(placeholderMesh)).toBe(true);
    });
  });

  describe("stripPlaygroundRendererForPuzzleKind", () => {
    const sample = [
      "shell-before",
      "//#region 🔖Puzzle3dPlayHost",
      "host-3d",
      "//#endregion 🔖Puzzle3dPlayHost",
      "//#region 🔖Puzzle5dPlayHost",
      "host-5d",
      "//#endregion 🔖Puzzle5dPlayHost",
      "//#region 🔖Puzzle2dPlayHost",
      "host-2d",
      "//#endregion 🔖Puzzle2dPlayHost",
      "//#region 🔖Boot",
      "boot-shared",
      "//#region 🧪Tests",
      "tests",
    ].join("\n");

    it("keeps only the requested puzzle host between shell and boot", () => {
      expect(stripPlaygroundRendererForPuzzleKind(sample, "2d")).toBe(
        ["shell-before", "//#region 🔖Puzzle2dPlayHost", "host-2d", "//#endregion 🔖Puzzle2dPlayHost", "//#region 🔖Boot", "boot-shared"].join("\n"),
      );
      expect(stripPlaygroundRendererForPuzzleKind(sample, "3d")).toContain("host-3d");
      expect(stripPlaygroundRendererForPuzzleKind(sample, "3d")).not.toContain("host-5d");
      expect(stripPlaygroundRendererForPuzzleKind(sample, "5d")).toContain("host-5d");
      expect(stripPlaygroundRendererForPuzzleKind(sample, "5d")).not.toContain("host-2d");
    });

    it("puzzle-2d virtual entry has no duplicate @puzzle/2d/react named imports", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "2d");
      expect(duplicateNamedImportsForModule(stripped, "@puzzle/2d/react")).toEqual([]);
    });

    it("wires virtual entry imports registerTabIcon when host registration calls it", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "wires");
      expect(stripped.includes("registerTabIcon(")).toBe(true);
      expect(stripped).toMatch(
        /import\s*\{[^}]*registerTabIcon[^}]*\}\s*from\s*["']@framework\/platform\/renderer\/react["']/,
      );
    });
  });
}
//#endregion 🔖ViteElementsAssets
