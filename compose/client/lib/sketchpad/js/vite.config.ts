// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build and development configuration for the sketchpad app.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite build configuration for the sketchpad application.
// Configuration MUST include MDX, React, WASM, and Tailwind CSS plugins.

// #region 🔌Adapters
import { readFileSync } from "node:fs";
import mdx from "@mdx-js/rollup";
import react from "@vitejs/plugin-react";
import path from "path";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { fileURLToPath } from "url";
import { defineConfig, type Plugin } from "vite";
import topLevelAwait from "__KEEP_vite_plugin__-top-level-await";
import wasm from "__KEEP_vite_plugin__-wasm";
import { meshCollectionVitePlugin, semioFaviconVitePlugin, uiAssetsVitePlugin, createWorkspaceViteResolveConfig, findWorkspacePackages, type PlaygroundAssetSpec } from "../../../../../ui/styling/vite-elements-assets.ts";
import { readInitialKitFixtureFromPath } from "../../../../fixture/script.ts";
// #endregion 🔌Adapters

/** @emoji 🧊 Kit mesh GLB serving spec for puzzle 3d world views embedded in the sketchpad app — mirrors
 * puzzle/plugin/rs's `[[package.metadata.semio.assets]]` `mesh-collection` row (see {@link meshCollectionVitePlugin}). */
const PUZZLE_3D_MESH_ASSET_SPEC: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
  kind: "mesh-collection",
  route: "/mesh",
  roots: ["asset/metabolism/representation", "asset/abbau-aufbau"],
  placeholder: "asset/mesh/placeholder.glb",
  filterFromExamples: true,
};

type CjsFacadeResolveOpts = {
  shimMain: string;
  shimWithSelector: string;
  schedulerEntry: string;
};

/** @emoji 🧱 Pre-transforms workspace TypeScript sources so Rollup import analysis accepts JSX and types. */
function monorepoWorkspaceTransformPlugin(workspaceRoot: string): Plugin {
  const root = workspaceRoot.replace(/\\/g, "/");
  return {
    name: "compose-monorepo-workspace-transform",
    enforce: "pre",
    async transform(code, id) {
      const file = id.replace(/\\/g, "/");
      if (file.includes("/node_modules/")) return;
      const allowed =
        file.startsWith(`${root}/framework/`) ||
        file.startsWith(`${root}/puzzle/`) ||
        file.startsWith(`${root}/ui/react/`) ||
        file.startsWith(`${root}/cad/`) ||
        file.startsWith(`${root}/compose/client/lib/sketchpad/`) ||
        file.startsWith(`${root}/compose/client/lib/react/`) ||
        file.startsWith(`${root}/asset/`) ||
        file.startsWith(`${root}/framework/product/playground/`) ||
        file.startsWith(`${root}/infinite/`) ||
        file.startsWith(`${root}/gis/`) ||
        file.startsWith(`${root}/reasoning/`);
      if (!allowed) return;
      if (!/\.(tsx?|mts|cts)$/.test(file)) return;
      const loader = file.endsWith(".tsx") || (file.endsWith(".ts") && /<[A-Za-z/]/.test(code)) ? "tsx" : "ts";
      const esbuild = await import("esbuild");
      const result = await esbuild.transform(code, {
        loader,
        jsx: "automatic",
        format: "esm",
        sourcefile: id,
        target: "es2022",
      });
      return { code: result.code, map: result.map || undefined };
    },
  };
}

const EMBEDDED_NODE_TEST_REGIONS = [/\/\/#region 🧪Tests[\s\S]*?\/\/#endregion 🧪Tests\s*/, /\/\/#region 🧪E2E[\s\S]*?\/\/#endregion 🧪E2E\s*/];

function isSketchpadIndexModule(id: string): boolean {
  return id.replace(/\\/g, "/").endsWith("/compose/client/lib/sketchpad/js/index.ts");
}

function isComposeJsIndexModule(id: string): boolean {
  return id.replace(/\\/g, "/").endsWith("/compose/client/lib/js/index.ts");
}

function isEmbeddedNodeTestIndexModule(id: string): boolean {
  return isSketchpadIndexModule(id) || isComposeJsIndexModule(id);
}

/** @emoji ✂️ Drops embedded vitest + Playwright regions from the browser bundle (Node runners use source + pw-loader). */
function stripEmbeddedNodeTests(source: string): string {
  let next = source;
  for (const region of EMBEDDED_NODE_TEST_REGIONS) {
    next = next.replace(region, "");
  }
  return next;
}

/** @emoji ✂️ Drops embedded vitest + Playwright regions from the browser bundle (Node runners use source + pw-loader). */
function stripEmbeddedNodeTestsPlugin(): Plugin {
  return {
    name: "compose-sketchpad-strip-embedded-node-tests",
    enforce: "pre",
    load(id) {
      if (!isEmbeddedNodeTestIndexModule(id)) return;
      return stripEmbeddedNodeTests(readFileSync(id, "utf8"));
    },
    transform(code, id) {
      if (!isEmbeddedNodeTestIndexModule(id)) return;
      const next = stripEmbeddedNodeTests(code);
      if (next === code) return;
      return { code: next, map: null };
    },
  };
}

function reactCjsFacadeResolvePlugin(opts: CjsFacadeResolveOpts): Plugin {
  return {
    name: "compose-react-cjs-facades",
    enforce: "pre",
    resolveId(id) {
      const n = id.replace(/\\/g, "/");
      if (n === "use-sync-external-store/shim/with-selector" || n === "use-sync-external-store/shim/with-selector.js") {
        return opts.shimWithSelector;
      }
      if (n === "use-sync-external-store/shim" || n === "use-sync-external-store/shim.js") {
        return opts.shimMain;
      }
      if (n === "scheduler" || (n.includes("scheduler/index.js") && !n.includes("/cjs/scheduler."))) {
        return opts.schedulerEntry;
      }
      return undefined;
    },
  };
}

const PLAYWRIGHT_DEV_STUB_ID = "\0compose-sketchpad-playwright-dev-stub";
const VITEST_DEV_STUB_ID = "\0compose-sketchpad-vitest-dev-stub";
const TESTING_LIBRARY_DEV_STUB_ID = "\0compose-sketchpad-testing-library-dev-stub";

/** @emoji 🧱 Keeps Playwright out of the browser dev graph when embedded E2E regions are scanned. */
function monorepoPlaywrightDevStubPlugin(): Plugin {
  return {
    name: "compose-sketchpad-playwright-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "@playwright/test" || id === "playwright" || id === "playwright-core") return PLAYWRIGHT_DEV_STUB_ID;
      return undefined;
    },
    load(id) {
      if (id !== PLAYWRIGHT_DEV_STUB_ID) return;
      return "export default {}; export const test = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} });";
    },
  };
}

/** @emoji 🧱 Keeps vitest and testing-library out of the browser dev graph when embedded test regions are scanned. */
function monorepoVitestDevStubPlugin(): Plugin {
  return {
    name: "compose-sketchpad-vitest-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "vitest" || id.startsWith("vitest/") || id.startsWith("@vitest/")) return VITEST_DEV_STUB_ID;
      if (id === "@testing-library/react" || id.startsWith("@testing-library/")) return TESTING_LIBRARY_DEV_STUB_ID;
      return undefined;
    },
    load(id) {
      if (id === VITEST_DEV_STUB_ID) {
        return "export default {}; export const describe = () => {}; export const it = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} }); export const vi = { fn: () => {}, mock: () => {}, spyOn: () => {} };";
      }
      if (id === TESTING_LIBRARY_DEV_STUB_ID) {
        return "export default {}; export const render = () => ({}); export const screen = {}; export const fireEvent = {}; export const waitFor = async (fn) => fn();";
      }
    },
  };
}

/** @emoji 🔀 Pre-resolves monorepo workspace package names before import-analysis. */
function monorepoWorkspaceResolvePlugin(aliases: Array<{ find: string | RegExp; replacement: string }>): Plugin {
  const exact = new Map<string, string>();
  for (const alias of aliases) {
    if (typeof alias.find === "string") exact.set(alias.find, alias.replacement);
  }
  return {
    name: "compose-monorepo-workspace-resolve",
    enforce: "pre",
    resolveId(id) {
      return exact.get(id);
    },
  };
}

/**
 * Absolute file path of the current module.
 * Path MUST be derived from import.meta.url.
 **/
const __filename = fileURLToPath(import.meta.url);
/**
 * Absolute directory path of the current module.
 * Path MUST be derived from __filename.
 **/
const __dirname = path.dirname(__filename);
const RUNTIME_ASSET_DIRECTORIES = new Set(["badge", "cursor", "font", "icon", "image", "logo", "representation", "compose"]);

function attachWasmAndAssetsMiddleware(server: { middlewares: { use: (fn: (req: any, res: any, next: any) => void) => void } }, fsMod: typeof import("fs")) {
  const sketchpadPublicPath = path.resolve(__dirname, "public");
  const assetsPath = path.resolve(__dirname, "../../../../../asset");
  const fixturesPath = path.resolve(__dirname, "../../../../fixture");
  server.middlewares.use((req: any, res: any, next: any) => {
    if (req.url?.endsWith(".wasm")) {
      const wasmFile = path.join(sketchpadPublicPath, req.url);
      if (fsMod.existsSync(wasmFile) && fsMod.statSync(wasmFile).isFile()) {
        res.setHeader("Content-Type", "application/wasm");
        fsMod.createReadStream(wasmFile).pipe(res);
        return;
      }
    }
    if (req.url?.startsWith("/fixture/")) {
      const requestedFixturePath = req.url.replace("/fixture/", "").split(/[?#]/, 1)[0];
      if (requestedFixturePath && !requestedFixturePath.includes("..")) {
        const filePath = path.join(fixturesPath, requestedFixturePath);
        if (fsMod.existsSync(filePath) && fsMod.statSync(filePath).isFile()) {
          if (requestedFixturePath.endsWith(".json")) {
            res.setHeader("Content-Type", "application/json");
          }
          const kitDir = path.dirname(filePath);
          if (requestedFixturePath.endsWith("/kit.compose.json") && (fsMod.existsSync(path.join(kitDir, "type")) || fsMod.existsSync(path.join(kitDir, "types")))) {
            const assembled = readInitialKitFixtureFromPath(filePath);
            res.end(JSON.stringify(assembled));
            return;
          }
          fsMod.createReadStream(filePath).pipe(res);
          return;
        }
      }
    }
    if (req.url?.startsWith("/asset/")) {
      const requestedAssetPath = req.url.replace("/asset/", "").split(/[?#]/, 1)[0];
      const [assetDirectory] = requestedAssetPath.split("/");
      if (!RUNTIME_ASSET_DIRECTORIES.has(assetDirectory)) {
        next();
        return;
      }
      const filePath = path.join(assetsPath, requestedAssetPath);
      if (fsMod.existsSync(filePath) && fsMod.statSync(filePath).isFile()) {
        if (requestedAssetPath.endsWith(".woff2")) {
          res.setHeader("Content-Type", "font/woff2");
        }
        fsMod.createReadStream(filePath).pipe(res);
        return;
      }
    }
    next();
  });
}

// Vite configuration with plugins, resolve aliases, and asset serving.
// Export MUST call defineConfig with the complete build configuration.
export default defineConfig(async ({ mode }) => {
  // 📥normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite");
  const fs = await import("fs");
  const workspaceRoot = path.resolve(__dirname, "../../../../../");
  const prod = mode === "production";
  const useSyncRoot = path.resolve(workspaceRoot, "node_modules/use-sync-external-store/cjs");
  const shimMain = path.join(useSyncRoot, prod ? "use-sync-external-store-shim.production.js" : "use-sync-external-store-shim.development.js");
  const shimWithSelector = path.join(useSyncRoot, "use-sync-external-store-shim", prod ? "with-selector.production.js" : "with-selector.development.js");
  const schedulerRoot = path.resolve(workspaceRoot, "node_modules/scheduler/cjs");
  const schedulerEntry = path.join(schedulerRoot, prod ? "scheduler.production.js" : "scheduler.development.js");
  const viteInternalFallback = path.resolve(workspaceRoot, "node_modules/vite/dist/node/index.js");
  const workspaceResolve = createWorkspaceViteResolveConfig(workspaceRoot);
  const workspaceAliases: Array<{ find: string | RegExp; replacement: string }> = [
    { find: "@semio-tech/compose-sketchpad", replacement: path.resolve(__dirname) },
    { find: "@compose/studio", replacement: path.resolve(__dirname, "../../studio") },
    { find: "@semio-tech/semio-asset/icon", replacement: path.resolve(workspaceRoot, "asset/index.ts") },
    ...(workspaceResolve.resolve?.alias ?? []),
    { find: /^use-sync-external-store\/shim\/with-selector(?:\.js)?$/, replacement: shimWithSelector },
    { find: /^use-sync-external-store\/shim(?:\.js)?$/, replacement: shimMain },
    { find: "scheduler", replacement: schedulerEntry },
    { find: "vite/internal", replacement: viteInternalFallback },
  ];
  const defaultPreloadKits = process.env.COMPOSE_SKETCHPAD_PRELOAD_KITS ?? (process.env.NODE_ENV !== "production" ? "/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json" : "");
  return {
    define: {
      __COMPOSE_JS_RUN_BENCHMARKS__: "false",
      __COMPOSE_JS_RUN_EMBEDDED_TESTS__: "false",
      __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
      "import.meta.env.COMPOSE_SKETCHPAD_PRELOAD_KITS": JSON.stringify(defaultPreloadKits),
    },
    resolve: {
      dedupe: ["react", "react-dom", "scheduler", "use-sync-external-store", "three", "@react-three/fiber", "@react-three/drei", "@radix-ui/react-compose-refs", "@radix-ui/react-slot"],
      alias: workspaceAliases,
    },
    server: workspaceResolve.server,
    plugins: [
      ...uiAssetsVitePlugin(path.resolve(workspaceRoot, "ui/asset")),
      ...semioFaviconVitePlugin(workspaceRoot),
      ...meshCollectionVitePlugin(workspaceRoot, PUZZLE_3D_MESH_ASSET_SPEC),
      monorepoPlaywrightDevStubPlugin(),
      monorepoVitestDevStubPlugin(),
      monorepoWorkspaceResolvePlugin(workspaceAliases),
      stripEmbeddedNodeTestsPlugin(),
      monorepoWorkspaceTransformPlugin(workspaceRoot),
      reactCjsFacadeResolvePlugin({ shimMain, shimWithSelector, schedulerEntry }),
      tailwind.default(),
      {
        ...mdx({
          remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
          rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
          providerImportSource: "@mdx-js/react",
        }),
        enforce: "pre",
      },
      react({ include: [/\.(tsx?|jsx?)$/, /[\\/]puzzle[\\/].*\.tsx$/] }),
      wasm(),
      topLevelAwait(), // needed for older browsers to run wasm
      {
        name: "serve-wasm-and-assets",
        enforce: "pre" as const,
        configureServer(server: any) {
          attachWasmAndAssetsMiddleware(server, fs);
        },
        configurePreviewServer(server: any) {
          attachWasmAndAssetsMiddleware(server, fs);
        },
      },
    ],
    optimizeDeps: {
      include: ["golden-layout", "scheduler", "use-sync-external-store/shim", "use-sync-external-store/shim/with-selector", "use-sync-external-store/with-selector", "@react-three/fiber", "@react-three/drei"],
      exclude: [
        "@semio-tech/compose-js",
        "@semio-tech/compose-sketchpad",
        "@playwright/test",
        "playwright",
        "playwright-core",
        "chromium-bidi",
        "fsevents",
        "vitest",
        "@vitest/expect",
        "@testing-library/react",
        ...findWorkspacePackages(workspaceRoot),
      ],
      esbuildOptions: {
        target: "es2020",
        plugins: [
          {
            name: "compose-sketchpad-strip-embedded-node-tests-depcrawl",
            setup(build) {
              build.onLoad({ filter: /index\.ts$/ }, (args) => {
                if (!isEmbeddedNodeTestIndexModule(args.path)) return;
                return {
                  contents: stripEmbeddedNodeTests(readFileSync(args.path, "utf8")),
                  loader: "ts",
                };
              });
            },
          },
        ],
      },
    },
    server: {
      host: "0.0.0.0",
      port: 5173,
    },
    build: {
      /** Workers + wasm-bindgen glue may use syntax older `esbuild` targets cannot downlevel (see __KEEP_vite_plugin__-top-level-await). */
      target: "es2022",
      rollupOptions: {
        external: ["@playwright/test"],
      },
    },
    worker: {
      format: "es",
    },
    ssr: {
      noExternal: ["golden-layout"],
    },
  };
});
// #endregion 🗄️Configuration
