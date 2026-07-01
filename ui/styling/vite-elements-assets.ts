// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `ui/asset` at `/asset/*` (fonts, cursors, …). */
// #endregion 🧲Header

// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { cpSync, createReadStream, existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { isAbsolute, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Connect, Plugin } from "vite";
import { defineConfig, type UserConfig } from "vite";
import {
  PLAYGROUND_LOCKED_FIXTURE_ENV,
  PLAYGROUND_PORTS,
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundEmbedUrl,
  playgroundLockedFixtureIdFromEnv,
  playgroundPlayViteDefine,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
  type PlaygroundSiteKind,
} from "../../repo/lib/js/src/index.ts";
// #endregion 🔌Adapters

export {
  PLAYGROUND_PORTS,
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundEmbedUrl,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
  type PlaygroundSiteKind,
};

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
    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    res.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
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
    if (!req.url?.startsWith("/asset/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/asset/".length).split(/[?#]/, 1)[0] ?? "");
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

/** @emoji 📂 Kit fixture GLB roots for puzzle 3d `/mesh/*` URLs. */
export function puzzle3dKitMeshRoots(repoRoot: string): { readonly meshRoots: readonly string[]; readonly placeholderMesh: string } {
  const metabolismMeshCandidates = [
    resolve(repoRoot, "compose/fixture/kit/folder/metabolism/representation"),
    resolve(repoRoot, "compose/fixture/kit/folder/metabolism/representations"),
  ];
  const metabolismMeshRoot = metabolismMeshCandidates.find((candidate) => existsSync(candidate)) ?? metabolismMeshCandidates[0]!;
  return {
    meshRoots: [metabolismMeshRoot, resolve(repoRoot, "compose/fixture/kit/folder/abbau-aufbau")],
    placeholderMesh: resolve(repoRoot, "asset/mesh/placeholder.glb"),
  };
}

/** @emoji 🌐 Connect middleware: serve kit GLBs at `/mesh/<name>.glb` (first matching root wins). */
export function createPuzzle3dMeshesMiddleware(meshRoots: readonly string[], placeholderMesh: string): Connect.NextHandleFunction {
  const rootsResolved = meshRoots.map((root) => resolve(root));
  const placeholderResolved = resolve(placeholderMesh);
  return (req, res, next) => {
    if (!req.url?.startsWith("/mesh/")) {
      next();
      return;
    }
    const rawName = decodeURIComponent(req.url.slice("/mesh/".length).split(/[?#]/, 1)[0] ?? "");
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

const PUZZLE_3D_LOCKED_FIXTURE_JSON_REL: Readonly<Record<string, readonly string[]>> = {
  nakagin: [
    "puzzle/3d/fixture/nakagin-capsule-tower.3d.json",
    "puzzle/5d/fixture/nakagin-capsule-tower.5d.json",
  ],
  "concrete-forest": [
    "puzzle/3d/fixture/concrete-forest.3d.json",
    "puzzle/5d/fixture/concrete-forest.5d.json",
  ],
};

/** @emoji 🔎 Collects `/mesh/*.glb` basenames referenced anywhere in fixture JSON. */
export function puzzle3dMeshBasenamesInJson(value: unknown, out = new Set<string>()): Set<string> {
  if (typeof value === "string") {
    const match = /^\/mesh\/([^?#]+\.glb)$/i.exec(value.trim());
    if (match) {
      out.add(decodeURIComponent(match[1]!));
    }
    return out;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      puzzle3dMeshBasenamesInJson(item, out);
    }
    return out;
  }
  if (value && typeof value === "object") {
    for (const entry of Object.values(value as Record<string, unknown>)) {
      puzzle3dMeshBasenamesInJson(entry, out);
    }
  }
  return out;
}

/** @emoji 🔒 GLB basenames required by {@link PLAYGROUND_LOCKED_FIXTURE_ENV}, if set. */
export function puzzle3dLockedFixtureMeshBasenames(repoRoot: string): Set<string> | undefined {
  const fixtureId = playgroundLockedFixtureIdFromEnv();
  if (!fixtureId) {
    return undefined;
  }
  const relPaths = PUZZLE_3D_LOCKED_FIXTURE_JSON_REL[fixtureId];
  if (!relPaths?.length) {
    return undefined;
  }
  const basenames = new Set<string>();
  for (const rel of relPaths) {
    const filePath = resolve(repoRoot, rel);
    if (!existsSync(filePath)) {
      continue;
    }
    try {
      puzzle3dMeshBasenamesInJson(JSON.parse(readFileSync(filePath, "utf8")), basenames);
    } catch {
      continue;
    }
  }
  basenames.add("placeholder.glb");
  return basenames;
}

/** @emoji 📦 Copies kit GLBs into a static `dist/mesh` tree (optional basename filter). */
export function copyPuzzle3dKitGlbs(meshRoots: readonly string[], dest: string, onlyBasenames?: ReadonlySet<string>): void {
  mkdirSync(dest, { recursive: true });
  const copied = new Set<string>();
  const want = (basename: string) => !onlyBasenames || onlyBasenames.has(basename);
  for (const meshRoot of meshRoots) {
    if (!existsSync(meshRoot)) {
      continue;
    }
    for (const entry of readdirSync(meshRoot)) {
      if (!entry.endsWith(".glb") || !want(entry) || copied.has(entry)) {
        continue;
      }
      const src = resolve(meshRoot, entry);
      if (!statSync(src).isFile()) {
        continue;
      }
      cpSync(src, resolve(dest, entry));
      copied.add(entry);
    }
  }
}

/** @emoji 🧊 Vite: serve and copy kit meshes at `/mesh/*` for puzzle 3d play and sketchpad. */
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
        const dest = resolve(viteRoot, "dist", "mesh");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        copyPuzzle3dKitGlbs(meshRoots, dest, puzzle3dLockedFixtureMeshBasenames(repoRoot));
        if (existsSync(placeholderMesh)) {
          cpSync(placeholderMesh, resolve(dest, "placeholder.glb"));
        }
      },
    },
  ];
}

/** @emoji 🔖 Canonical semio emblem favicon `<link>` tags for playground and app `index.html` heads. */
export const SEMIO_FAVICON_HEAD_HTML =
  `<link rel="icon" href="./favicon.svg" type="image/svg+xml" />\n    <link rel="icon" href="./favicon.ico" sizes="any" />`;

/** @emoji 🔖 Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
export function semioFaviconSources(repoRoot: string): { readonly svg: string; readonly ico: string } {
  const logoRoot = resolve(repoRoot, "asset/logo");
  return {
    svg: resolve(logoRoot, "emblem_dark_round.svg"),
    ico: resolve(logoRoot, "favicon_dark_round_32x32.ico"),
  };
}

const SEMIO_FAVICON_BLEED_RECT = '<rect width="350" height="350" fill="#001117"/>';

/** @emoji 🔖 Favicon SVG with opaque bleed so ICO rasterization avoids white matte outside the round emblem. */
export function semioFaviconSvgMarkup(svgPath: string): string | undefined {
  if (!existsSync(svgPath)) {
    return undefined;
  }
  const raw = readFileSync(svgPath, "utf8");
  if (raw.includes(SEMIO_FAVICON_BLEED_RECT)) {
    return raw;
  }
  const open = raw.match(/<svg[^>]*>/)?.[0];
  if (!open) {
    return raw;
  }
  return raw.replace(open, `${open}${SEMIO_FAVICON_BLEED_RECT}`);
}

function createSemioFaviconMiddleware(favicons: { readonly svg: string; readonly ico: string }): Connect.NextHandleFunction {
  return (req, res, next) => {
    const url = req.url?.split(/[?#]/, 1)[0];
    if (url === "/favicon.svg") {
      const markup = semioFaviconSvgMarkup(favicons.svg);
      if (markup) {
        res.setHeader("Content-Type", "image/svg+xml");
        res.end(markup);
        return;
      }
    }
    if (url === "/favicon.ico" && existsSync(favicons.ico)) {
      res.setHeader("Content-Type", "image/x-icon");
      createReadStream(favicons.ico).pipe(res);
      return;
    }
    next();
  };
}

/** @emoji 🔖 Vite: serve and copy semio emblem favicons at `/favicon.svg` and `/favicon.ico`. */
export function semioFaviconVitePlugin(repoRoot: string): Plugin[] {
  const favicons = semioFaviconSources(repoRoot);
  const serveFavicon = createSemioFaviconMiddleware(favicons);
  let viteRoot = process.cwd();
  return [
    {
      name: "semio-favicon-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFavicon);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFavicon);
      },
    },
    {
      name: "semio-favicon-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dist = resolve(viteRoot, "dist");
        mkdirSync(dist, { recursive: true });
        const markup = semioFaviconSvgMarkup(favicons.svg);
        if (markup) {
          writeFileSync(resolve(dist, "favicon.svg"), markup);
        }
        if (existsSync(favicons.ico)) {
          cpSync(favicons.ico, resolve(dist, "favicon.ico"));
        }
      },
    },
  ];
}

/** @emoji 🌐 Vite: serve and copy `ui/asset` at `/asset/*` for palette fonts and cursors. */
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
        const dest = resolve(viteRoot, "dist", "asset");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🛝 Shared Vite preset for puzzle play harnesses (assets, renderer subpaths, workspace aliases). */
const PLAYGROUND_RENDERER_SHELL_SUBPATHS = ["@semio-tech/framework-playground-renderer-react/shell", "@semio-tech/framework-playground-renderer-react/boot"] as const;

const PLAYGROUND_RENDERER_PUZZLE_HOSTS_START = "//#region 🔖Puzzle3dPlayHost";
const PLAYGROUND_RENDERER_BOOT_START = "//#region 🔖Boot";
const PLAYGROUND_RENDERER_VITEST_START = "//#region 🧪Tests";

export type PlaygroundRendererPuzzleKind = "2d" | "3d" | "5d" | "map" | "flow" | "dag" | "trinity-jack" | "trinity-rewrite" | "procedural-3d" | "procedural-2d" | "presentation" | "wires" | "shooting" | "forms" | "raster" | "writer" | "semios";

const PLAYGROUND_RENDERER_PUZZLE_BOOT_SUBPATHS: Readonly<Record<string, PlaygroundRendererPuzzleKind>> = {
  "@semio-tech/framework-playground-renderer-react/puzzle/2d": "2d",
  "@semio-tech/framework-playground-renderer-react/puzzle/3d": "3d",
  "@semio-tech/framework-playground-renderer-react/puzzle/5d": "5d",
  "@semio-tech/framework-playground-renderer-react/puzzle/map": "map",
  "@semio-tech/framework-playground-renderer-react/flow": "flow",
  "@semio-tech/framework-playground-renderer-react/dag": "dag",
  "@semio-tech/framework-playground-renderer-react/trinity-jack": "trinity-jack",
  "@semio-tech/framework-playground-renderer-react/trinity-rewrite": "trinity-rewrite",
  "@semio-tech/framework-playground-renderer-react/procedural-3d": "procedural-3d",
  "@semio-tech/framework-playground-renderer-react/procedural-2d": "procedural-2d",
  "@semio-tech/framework-playground-renderer-react/shooting": "shooting",
  "@semio-tech/framework-playground-renderer-react/presentation": "presentation",
  "@semio-tech/framework-playground-renderer-react/forms": "forms",
  "@semio-tech/framework-playground-renderer-react/raster": "raster",
  "@semio-tech/framework-playground-renderer-react/writer": "writer",
  "@semio-tech/framework-playground-renderer-react/semios": "semios",
  "@semio-tech/framework-playground-renderer-react/reasoning/wires": "wires",
};

const PLAYGROUND_RENDERER_PUZZLE_HOST_MARKERS: Readonly<Record<PlaygroundRendererPuzzleKind, { readonly start: string; readonly end: string }>> = {
  "3d": { start: "//#region 🔖Puzzle3dPlayHost", end: "//#endregion 🔖Puzzle3dPlayHost" },
  "5d": { start: "//#region 🔖Puzzle5dPlayHost", end: "//#endregion 🔖Puzzle5dPlayHost" },
  "2d": { start: "//#region 🔖Puzzle2dPlayHost", end: "//#endregion 🔖Puzzle2dPlayHost" },
  map: { start: "//#region 🔖MapPlayHost", end: "//#endregion 🔖MapPlayHost" },
  flow: { start: "//#region 🔖FlowPlayHost", end: "//#endregion 🔖FlowPlayHost" },
  dag: { start: "//#region 🔖DagPlayHost", end: "//#endregion 🔖DagPlayHost" },
  "trinity-jack": { start: "//#region 🔖TrinityPlayHost", end: "//#endregion 🔖TrinityPlayHost" },
  "trinity-rewrite": { start: "//#region 🔖TrinityPlayHost", end: "//#endregion 🔖TrinityPlayHost" },
  "procedural-3d": { start: "//#region 🔖ProceduralPlayHost", end: "//#endregion 🔖ProceduralPlayHost" },
  "procedural-2d": { start: "//#region 🔖Procedural2dPlayHost", end: "//#endregion 🔖Procedural2dPlayHost" },
  shooting: { start: "//#region 🔖ShootingPlayHost", end: "//#endregion 🔖ShootingPlayHost" },
  presentation: { start: "//#region 🔖PresentationPlayHost", end: "//#endregion 🔖PresentationPlayHost" },
  forms: { start: "//#region 🔖FormsPlayHost", end: "//#endregion 🔖FormsPlayHost" },
  raster: { start: "//#region 🔖RasterPlayHost", end: "//#endregion 🔖RasterPlayHost" },
  writer: { start: "//#region 🔖WriterPlayHost", end: "//#endregion 🔖WriterPlayHost" },
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
      const puzzleMatch = id.match(/playgroundEntry=puzzle-([^&?]+)/);
      if (puzzleMatch && puzzleMatch[1] === "semios") {
        return source;
      }
      if (puzzleMatch && puzzleMatch[1] in PLAYGROUND_RENDERER_PUZZLE_HOST_MARKERS) {
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
export const GIS_MAP_TILE_USER_AGENT = "ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";

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

/** @emoji 🗺 `fetch` loads missing tiles at runtime; `bundle` serves only cached tiles and copies them into `dist` on build. */
export type GisMapTileServeMode = "fetch" | "bundle";

export const GIS_MAP_TILE_SERVE_MODE_ENV = "GIS_MAP_TILE_SERVE_MODE";

export function resolveGisMapTileServeMode(value?: string): GisMapTileServeMode {
  return value === "bundle" ? "bundle" : "fetch";
}

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
  log(
    `[gis/2d/play] prefetch ${jobs.length} tiles ${zoomLabel}` +
      (skipExisting ? ` (${skipped} cached, ${pending.length} to fetch)` : ""),
  );
  if (pending.length === 0) {
    log(`[gis/2d/play] prefetch done: downloaded=0 skipped=${skipped} failed=0`);
    return { downloaded: 0, skipped, failed: 0 };
  }
  let downloaded = 0;
  let failed = 0;
  const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));
  for (let i = 0; i < pending.length; i += concurrency) {
    const batch = pending.slice(i, i + concurrency);
    await Promise.all(
      batch.map(async (job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
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
    if (delayMs > 0 && i + concurrency < pending.length) {
      await sleep(delayMs);
    }
  }
  log(`[gis/2d/play] prefetch done: downloaded=${downloaded} skipped=${skipped} failed=${failed}`);
  return { downloaded, skipped, failed };
}
//#endregion 🔖MapTileCache

function createOsmTileMiddleware(cacheRoot: string, mode: GisMapTileServeMode): Connect.NextHandleFunction {
  return async (req, res, next) => {
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
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
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
}

function createVtTileMiddleware(cacheRoot: string, mode: GisMapTileServeMode): Connect.NextHandleFunction {
  return async (req, res, next) => {
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
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
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
}

/** @emoji 🗺 Vite plugins for GIS map raster/vector tiles (`fetch` or offline `bundle`). */
export function gisMapTilesVitePlugins(repoRoot: string, mode: GisMapTileServeMode = "fetch"): Plugin[] {
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const serveOsm = createOsmTileMiddleware(osm, mode);
  const serveVt = createVtTileMiddleware(vt, mode);
  let viteRoot = process.cwd();
  const plugins: Plugin[] = [
    {
      name: "gis-2d-osm-tiles",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveOsm);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveOsm);
      },
    },
    {
      name: "gis-2d-vt-tiles",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveVt);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveVt);
      },
    },
  ];
  if (mode === "bundle") {
    plugins.push({
      name: "gis-2d-tiles-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        const dist = resolve(viteRoot, "dist");
        mkdirSync(dist, { recursive: true });
        if (existsSync(osm)) {
          cpSync(osm, resolve(dist, "osm"), { recursive: true });
        }
        if (existsSync(vt)) {
          cpSync(vt, resolve(dist, "vt"), { recursive: true });
        }
      },
    });
  }
  return plugins;
}

/** @emoji 🗺 Dev/preview proxy for OpenStreetMap raster tiles at `/osm/:z/:x/:y.png`. */
export function osmTileProxyVitePlugin(cacheDir: string, mode: GisMapTileServeMode = "fetch"): Plugin {
  const { osm } = mapTileCacheRoots(cacheDir);
  const serveOsm = createOsmTileMiddleware(osm, mode);
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
export function mapLibreVectorTileProxyVitePlugin(cacheDir: string, mode: GisMapTileServeMode = "fetch"): Plugin {
  const { vt } = mapTileCacheRoots(cacheDir);
  const serveVt = createVtTileMiddleware(vt, mode);
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

/** @emoji 🦀 Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
export const FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/flow-module-core",
  "@semio-tech/flow-module-math",
  "@semio-tech/flow-module-text",
  "@semio-tech/flow-module-logic",
  "@semio-tech/flow-module-dictionary",
  "@semio-tech/flow-module-list",
  "@semio-tech/flow-module-brep",
  "@semio-tech/flow-module-draw",
] as const;

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
    { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "ui/react/index.tsx") },
    { find: "@semio-tech/ui-asset", replacement: resolve(repoRoot, "ui/asset/index.ts") },
    { find: "@semio-tech/infinite-cavas-react-renderer", replacement: resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx") },
    { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "infinite/world/r3f/index.tsx") },
    { find: "@semio-tech/puzzle-2d-play", replacement: resolve(repoRoot, "puzzle/2d/play/index.ts") },
    { find: "@semio-tech/puzzle-3d-play", replacement: resolve(repoRoot, "puzzle/3d/play/index.ts") },
    { find: "@semio-tech/puzzle-5d-play", replacement: resolve(repoRoot, "puzzle/5d/play/index.ts") },
    { find: "@semio-tech/puzzle-2d-react", replacement: resolve(repoRoot, "puzzle/2d/react/index.tsx") },
    { find: "@semio-tech/puzzle-3d-react", replacement: resolve(repoRoot, "puzzle/3d/react/index.tsx") },
    { find: "@semio-tech/puzzle-5d-react", replacement: resolve(repoRoot, "puzzle/5d/react/index.tsx") },
    { find: "@semio-tech/gis-2d-play", replacement: resolve(repoRoot, "gis/2d/play/index.ts") },
    { find: "@semio-tech/gis-2d-react", replacement: resolve(repoRoot, "gis/2d/react/index.tsx") },
    { find: "@semio-tech/reasoning-mindmap-wires-play", replacement: resolve(repoRoot, "reasoning/mindmap/wires/play/index.ts") },
    { find: "@semio-tech/reasoning-mindmap-wires-react", replacement: resolve(repoRoot, "reasoning/mindmap/wires/react/index.ts") },
    { find: "@semio-tech/reasoning-mindmap-react", replacement: resolve(repoRoot, "reasoning/mindmap/react/index.tsx") },
    { find: "@semio-tech/framework-presentation-play", replacement: resolve(repoRoot, "framework/product/presentation/play/index.ts") },
    { find: "@semio-tech/framework-presentation-core", replacement: resolve(repoRoot, "framework/product/presentation/core/index.ts") },
    { find: "@semio-tech/framework-presentation-renderer-react", replacement: resolve(repoRoot, "framework/product/presentation/renderer/react/index.tsx") },
    { find: "@semio-tech/flow-play", replacement: resolve(repoRoot, "flow/play/index.ts") },
    { find: "@semio-tech/flow-react", replacement: resolve(repoRoot, "flow/react/index.tsx") },
    { find: "@semio-tech/flow-module-core", replacement: resolve(repoRoot, "flow/module/core/pkg/flow_module_core.js") },
    { find: "@semio-tech/flow-module-brep", replacement: resolve(repoRoot, "flow/module/brep/pkg/flow_module_brep.js") },
    { find: "@semio-tech/flow-module-draw", replacement: resolve(repoRoot, "flow/module/draw/pkg/flow_module_draw.js") },
    { find: "@semio-tech/flow-module-math", replacement: resolve(repoRoot, "flow/module/math/pkg/flow_module_math.js") },
    { find: "@semio-tech/flow-module-text", replacement: resolve(repoRoot, "flow/module/text/pkg/flow_module_text.js") },
    { find: "@semio-tech/flow-module-logic", replacement: resolve(repoRoot, "flow/module/logic/pkg/flow_module_logic.js") },
    { find: "@semio-tech/flow-module-dictionary", replacement: resolve(repoRoot, "flow/module/dictionary/pkg/flow_module_dictionary.js") },
    { find: "@semio-tech/flow-module-list", replacement: resolve(repoRoot, "flow/module/list/pkg/flow_module_list.js") },
    { find: "@semio-tech/dag-play", replacement: resolve(repoRoot, "mathematical/graph/port/directed/dag/play/index.ts") },
    { find: "@semio-tech/dag-react", replacement: resolve(repoRoot, "mathematical/graph/port/directed/dag/react/index.tsx") },
    { find: "@semio-tech/trinity-jack-play", replacement: resolve(repoRoot, "trinity/jack/play/index.ts") },
    { find: "@semio-tech/trinity-rewrite-play", replacement: resolve(repoRoot, "trinity/rewrite/play/index.ts") },
    { find: "@semio-tech/trinity-react", replacement: resolve(repoRoot, "trinity/react/index.tsx") },
    { find: "@semio-tech/procedural-3d-play", replacement: resolve(repoRoot, "procedural/3d/play/index.ts") },
    { find: "@semio-tech/procedural-3d-react", replacement: resolve(repoRoot, "procedural/3d/react/index.tsx") },
    { find: "@semio-tech/procedural-2d-play", replacement: resolve(repoRoot, "procedural/2d/play/index.ts") },
    { find: "@semio-tech/procedural-2d-react", replacement: resolve(repoRoot, "procedural/2d/react/index.tsx") },
    { find: "@semio-tech/shooting-play", replacement: resolve(repoRoot, "shooting/play/index.ts") },
    { find: "@semio-tech/shooting-react", replacement: resolve(repoRoot, "shooting/react/index.tsx") },
    { find: "@semio-tech/kernel-3d-js", replacement: resolve(repoRoot, "kernel/3d/js/index.ts") },
    { find: "@semio-tech/kernel-2d-js", replacement: resolve(repoRoot, "kernel/2d/js/index.ts") },
  ];
}

/** @emoji 🖼️ Vite: serve and copy `cad/fixture` at `/cad-fixture/*` for CAD world reference planes. */
export function cadFixtureVitePlugin(repoRoot: string): Plugin[] {
  const fixtureRoot = resolve(repoRoot, "cad/fixture");
  const serveFixture: Connect.NextHandleFunction = (req, res, next) => {
    if (!req.url?.startsWith("/cad-fixture/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/cad-fixture/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    if (filePath.endsWith(".png")) {
      res.setHeader("Content-Type", "image/png");
    } else if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
      res.setHeader("Content-Type", "image/jpeg");
    } else if (filePath.endsWith(".pdf")) {
      res.setHeader("Content-Type", "application/pdf");
    } else if (filePath.endsWith(".svg")) {
      res.setHeader("Content-Type", "image/svg+xml");
    }
    createReadStream(filePath).pipe(res);
  };
  let viteRoot = process.cwd();
  return [
    {
      name: "cad-fixture-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: "cad-fixture-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "cad-fixture");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🌐 Vite: serve and copy `infinite/fixture` at `/infinite-fixture/*` for world reference planes. */
export function infiniteFixtureVitePlugin(repoRoot: string): Plugin[] {
  const fixtureRoot = resolve(repoRoot, "infinite/fixture");
  const serveFixture: Connect.NextHandleFunction = (req, res, next) => {
    if (!req.url?.startsWith("/infinite-fixture/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/infinite-fixture/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    if (filePath.endsWith(".png")) {
      res.setHeader("Content-Type", "image/png");
    } else if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
      res.setHeader("Content-Type", "image/jpeg");
    } else if (filePath.endsWith(".pdf")) {
      res.setHeader("Content-Type", "application/pdf");
    } else if (filePath.endsWith(".svg")) {
      res.setHeader("Content-Type", "image/svg+xml");
    }
    createReadStream(filePath).pipe(res);
  };
  let viteRoot = process.cwd();
  return [
    {
      name: "infinite-fixture-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: "infinite-fixture-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        viteRoot = config.root;
      },
      closeBundle() {
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(viteRoot, "dist", "infinite-fixture");
        mkdirSync(resolve(viteRoot, "dist"), { recursive: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🛝 `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
  const { playDir, repoRoot, playEntryKind, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } = options;
  const uiAssetsRoot = resolve(repoRoot, "ui/asset");
  const rendererRoot = resolve(repoRoot, "framework/product/playground/renderer/react");
  const rendererIndex = resolve(rendererRoot, "index.tsx");
  const rendererAliases = playgroundRendererResolveAliases(repoRoot);
  return defineConfig({
    root: playDir,
    base: "./",
    publicDir: resolve(playDir, "public"),
    assetsInclude: ["**/*.wasm"],
    worker: { format: "es" },
    define: {
      ...playgroundPlayViteDefine(
        playEntryKind ? { "import.meta.env.PUZZLE_PLAY_ENTRY": JSON.stringify(playEntryKind) } : {},
      ),
    },
    plugins: [
      ...uiAssetsVitePlugin(uiAssetsRoot),
      ...semioFaviconVitePlugin(repoRoot),
      ...cadFixtureVitePlugin(repoRoot),
      infiniteFixtureVitePlugin(repoRoot),
      ...(playEntryKind === "3d" || playEntryKind === "5d" || playEntryKind === "shooting" ? puzzle3dMeshesVitePlugin(repoRoot) : []),
      ...(playEntryKind === "map"
        ? gisMapTilesVitePlugins(repoRoot, resolveGisMapTileServeMode(process.env[GIS_MAP_TILE_SERVE_MODE_ENV]))
        : []),
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

  describe("resolveGisMapTileServeMode", () => {
    it("defaults to fetch", () => {
      expect(resolveGisMapTileServeMode(undefined)).toBe("fetch");
      expect(resolveGisMapTileServeMode("")).toBe("fetch");
      expect(resolveGisMapTileServeMode("online")).toBe("fetch");
    });

    it("selects bundle only for bundle", () => {
      expect(resolveGisMapTileServeMode("bundle")).toBe("bundle");
    });
  });

  describe("gisMapTilesVitePlugins", () => {
    it("adds a build copy plugin only for bundle mode", () => {
      const fetchPlugins = gisMapTilesVitePlugins(repoRoot, "fetch");
      const bundlePlugins = gisMapTilesVitePlugins(repoRoot, "bundle");
      expect(fetchPlugins.some((plugin) => plugin.name === "gis-2d-tiles-build")).toBe(false);
      expect(bundlePlugins.some((plugin) => plugin.name === "gis-2d-tiles-build")).toBe(true);
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

  describe("prefetchMapTiles", () => {
    it("skips tiles already present in cache without fetching", async () => {
      const { osm } = mapTileCacheRoots(repoRoot);
      const tile = { z: 0, x: 0, y: 0 };
      const filePath = resolve(osm, `${tile.z}/${tile.x}/${tile.y}.png`);
      const hadCache = existsSync(filePath);
      if (!hadCache) {
        mkdirSync(resolve(filePath, ".."), { recursive: true });
        writeFileSync(filePath, Buffer.from([0x89, 0x50, 0x4e, 0x47]));
      }
      const lines: string[] = [];
      const result = await prefetchMapTiles({
        repoRoot,
        bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
        raster: true,
        vector: false,
        zMinRaster: 0,
        zMaxRaster: 0,
        concurrency: 4,
        delayMs: 0,
        log: (line) => lines.push(line),
      });
      expect(result.skipped).toBeGreaterThan(0);
      expect(result.downloaded).toBe(0);
      expect(lines.some((line) => line.includes("cached"))).toBe(true);
      if (!hadCache) {
        const { unlinkSync } = await import("node:fs");
        unlinkSync(filePath);
      }
    });
  });

  describe("semioFaviconVitePlugin", () => {
    it("points at round dark emblem svg and ico under asset/logo", () => {
      const { svg, ico } = semioFaviconSources(repoRoot);
      expect(svg).toBe(resolve(repoRoot, "asset/logo/emblem_dark_round.svg"));
      expect(ico).toBe(resolve(repoRoot, "asset/logo/favicon_dark_round_32x32.ico"));
      expect(existsSync(svg)).toBe(true);
      expect(existsSync(ico)).toBe(true);
    });

    it("registers serve and build plugins", () => {
      const plugins = semioFaviconVitePlugin(repoRoot);
      expect(plugins.map((plugin) => plugin.name)).toEqual(["semio-favicon-serve", "semio-favicon-build"]);
    });

    it("injects opaque bleed into round dark favicon svg", () => {
      const { svg } = semioFaviconSources(repoRoot);
      const markup = semioFaviconSvgMarkup(svg);
      expect(markup).toContain('<rect width="350" height="350" fill="#001117"/>');
    });
  });

  describe("puzzle3dKitMeshRoots", () => {
    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      const { meshRoots, placeholderMesh } = puzzle3dKitMeshRoots(repoRoot);
      expect(existsSync(resolve(meshRoots[0]!, "capsule_J.glb"))).toBe(true);
      expect(existsSync(resolve(meshRoots[0]!, "capsule-with-balcony_slash.glb"))).toBe(true);
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

    it("puzzle-2d virtual entry has no duplicate @semio-tech/puzzle-2d-react named imports", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "2d");
      expect(duplicateNamedImportsForModule(stripped, "@semio-tech/puzzle-2d-react")).toEqual([]);
    });

    it("wires virtual entry imports registerTabIcon when host registration calls it", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "wires");
      expect(stripped.includes("registerTabIcon(")).toBe(true);
      expect(stripped).toMatch(
        /import\s*\{[^}]*registerTabIcon[^}]*\}\s*from\s*["']@framework\/platform\/renderer\/react["']/,
      );
    });

    it("forms virtual entry keeps only the forms host slice", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "forms");
      expect(stripped).toContain("//#region 🔖FormsPlayHost");
      expect(stripped).toContain("bootFormsPlay");
      expect(stripped).not.toContain("//#region 🔖ShootingPlayHost");
      expect(stripped).not.toContain("//#region 🔖Puzzle3dPlayHost");
    });

    it("writer virtual entry imports WriterCanvas in the host slice", () => {
      const rendererIndex = resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx");
      const stripped = stripPlaygroundRendererForPuzzleKind(readFileSync(rendererIndex, "utf8"), "writer");
      expect(stripped).toContain("//#region 🔖WriterPlayHost");
      expect(stripped).toContain("bootWriterPlay");
      expect(stripped).toMatch(/import\s*\{[^}]*WriterCanvas[^}]*\}\s*from\s*["']@semio-tech\/writer-react["']/);
      expect(stripped).toMatch(/import\s*\{[^}]*createWriterDocument[^}]*\}\s*from\s*["']@semio-tech\/writer-core["']/);
      expect(stripped).toMatch(/import\s*\{[^}]*createJackLspWorker[^}]*\}\s*from\s*["']@semio-tech\/trinity-react["']/);
      expect(stripped).not.toContain("//#region 🔖RasterPlayHost");
    });
  });
}
//#endregion 🔖ViteElementsAssets
