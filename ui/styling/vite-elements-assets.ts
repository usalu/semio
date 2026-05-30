// #region 🧲Header
/** @emoji 🌐 Vite plugin: serve and copy `ui/assets` at `/assets/*` (fonts, cursors, …). */
// #endregion 🧲Header

// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { cpSync, createReadStream, existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";
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

export type PlaygroundRendererPuzzleKind = "2d" | "3d" | "5d";

const PLAYGROUND_RENDERER_PUZZLE_BOOT_SUBPATHS: Readonly<Record<string, PlaygroundRendererPuzzleKind>> = {
  "@framework/playground/renderer/react/puzzle/2d": "2d",
  "@framework/playground/renderer/react/puzzle/3d": "3d",
  "@framework/playground/renderer/react/puzzle/5d": "5d",
};

const PLAYGROUND_RENDERER_PUZZLE_HOST_MARKERS: Readonly<Record<PlaygroundRendererPuzzleKind, { readonly start: string; readonly end: string }>> = {
  "3d": { start: "//#region 🔖Puzzle3dPlayHost", end: "//#endregion 🔖Puzzle3dPlayHost" },
  "5d": { start: "//#region 🔖Puzzle5dPlayHost", end: "//#endregion 🔖Puzzle5dPlayHost" },
  "2d": { start: "//#region 🔖Puzzle2dPlayHost", end: "//#endregion 🔖Puzzle2dPlayHost" },
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
      const puzzleMatch = id.match(/playgroundEntry=puzzle-(2d|3d|5d)/);
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

/** @emoji 🛝 `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
  const { playDir, repoRoot, playEntryKind, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } = options;
  const uiAssetsRoot = resolve(repoRoot, "ui/assets");
  const rendererRoot = resolve(repoRoot, "framework/product/playground/renderer/react");
  const playgroundCore = resolve(repoRoot, "framework/product/playground/core/core.ts");
  const platformCore = resolve(repoRoot, "framework/product/platform/core/index.ts");
  const platformRenderer = resolve(repoRoot, "framework/product/platform/renderer/react/index.tsx");
  const frameworkCore = resolve(repoRoot, "framework/core/index.ts");
  const uiReact = resolve(repoRoot, "ui/react/index.tsx");
  const rendererIndex = resolve(rendererRoot, "index.tsx");
  const rendererAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = [
    { find: /^@framework\/playground\/renderer\/react$/, replacement: rendererIndex },
    { find: /^@framework\/playground\/core$/, replacement: playgroundCore },
    { find: /^@framework\/platform\/core$/, replacement: platformCore },
    { find: /^@framework\/platform\/renderer\/react$/, replacement: platformRenderer },
    { find: /^@framework\/core$/, replacement: frameworkCore },
    { find: "@ui/react", replacement: uiReact },
    { find: "@puzzle/2d/play", replacement: resolve(repoRoot, "puzzle/2d/play/index.ts") },
    { find: "@puzzle/3d/play", replacement: resolve(repoRoot, "puzzle/3d/play/index.ts") },
    { find: "@puzzle/5d/play", replacement: resolve(repoRoot, "puzzle/5d/play/index.ts") },
    { find: "@puzzle/2d/react", replacement: resolve(repoRoot, "puzzle/2d/react/index.tsx") },
    { find: "@puzzle/3d/react", replacement: resolve(repoRoot, "puzzle/3d/react/index.tsx") },
    { find: "@puzzle/5d/react", replacement: resolve(repoRoot, "puzzle/5d/react/index.tsx") },
  ];
  return defineConfig({
    root: playDir,
    base: "./",
    publicDir: resolve(playDir, "public"),
    assetsInclude: ["**/*.wasm"],
    worker: { format: "es" },
    define: playEntryKind ? { "import.meta.env.PUZZLE_PLAY_ENTRY": JSON.stringify(playEntryKind) } : undefined,
    plugins: [...uiAssetsVitePlugin(uiAssetsRoot), tailwindcss(), react(), playgroundIframeEmbedHeadersPlugin(), playgroundRendererShellEntryPlugin(rendererIndex), ...extraPlugins],
    build: playgroundStaticSiteBuildOptions(build),
    server: {
      fs: { allow: [repoRoot] },
      ...(watchIgnored ? { watch: { ignored: watchIgnored } } : {}),
      ...server,
    },
    resolve: {
      alias: [...rendererAliases, ...extraAliases],
      ...(resolveDedupe ? { dedupe: [...resolveDedupe] } : {}),
    },
    ...(optimizeDeps ? { optimizeDeps } : {}),
  });
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

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
  });
}
//#endregion 🔖ViteElementsAssets
