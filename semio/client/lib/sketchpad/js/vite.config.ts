// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build and development configuration for the sketchpad app.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite build configuration for the sketchpad application.
// Configuration MUST include MDX, React, WASM, and Tailwind CSS plugins.

// #region 🔌Adapters
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
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
// #endregion 🔌Adapters

type CjsFacadeResolveOpts = {
  shimMain: string;
  shimWithSelector: string;
  schedulerEntry: string;
};

/** @emoji 🧱 Pre-transforms workspace TypeScript sources so Rollup import analysis accepts JSX and types. */
function monorepoWorkspaceTransformPlugin(workspaceRoot: string): Plugin {
  const root = workspaceRoot.replace(/\\/g, "/");
  return {
    name: "semio-monorepo-workspace-transform",
    enforce: "pre",
    async transform(code, id) {
      const file = id.replace(/\\/g, "/");
      if (file.includes("/node_modules/")) return;
      const allowed =
        file.startsWith(`${root}/framework/`) ||
        file.startsWith(`${root}/puzzle/`) ||
        file.startsWith(`${root}/ui/react/`) ||
        file.startsWith(`${root}/cad/`) ||
        file.startsWith(`${root}/semio/client/lib/sketchpad/`) ||
        file.startsWith(`${root}/semio/client/lib/react/`) ||
        file.startsWith(`${root}/semio/assets/`) ||
        file.startsWith(`${root}/framework/product/playground/`);
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

function reactCjsFacadeResolvePlugin(opts: CjsFacadeResolveOpts): Plugin {
  return {
    name: "semio-react-cjs-facades",
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
const RUNTIME_ASSET_DIRECTORIES = new Set(["badges", "cursors", "fonts", "icons", "images", "logo", "representations"]);

function attachWasmAndAssetsMiddleware(server: { middlewares: { use: (fn: (req: any, res: any, next: any) => void) => void } }, fsMod: typeof import("fs")) {
  const sketchpadPublicPath = path.resolve(__dirname, "public");
  const assetsPath = path.resolve(__dirname, "../../../../assets");
  server.middlewares.use((req: any, res: any, next: any) => {
    if (req.url?.endsWith(".wasm")) {
      const wasmFile = path.join(sketchpadPublicPath, req.url);
      if (fsMod.existsSync(wasmFile) && fsMod.statSync(wasmFile).isFile()) {
        res.setHeader("Content-Type", "application/wasm");
        fsMod.createReadStream(wasmFile).pipe(res);
        return;
      }
    }
    if (req.url?.startsWith("/assets/")) {
      const requestedAssetPath = req.url.replace("/assets/", "").split(/[?#]/, 1)[0];
      const [assetDirectory] = requestedAssetPath.split("/");
      if (!RUNTIME_ASSET_DIRECTORIES.has(assetDirectory)) {
        next();
        return;
      }
      const filePath = path.join(assetsPath, requestedAssetPath);
      if (fsMod.existsSync(filePath) && fsMod.statSync(filePath).isFile()) {
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
  return {
    define: {
      __SEMIO_JS_RUN_BENCHMARKS__: "false",
      __SEMIO_JS_RUN_EMBEDDED_TESTS__: "false",
      __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
    },
    resolve: {
      dedupe: ["react", "react-dom", "scheduler", "use-sync-external-store"],
      alias: [
        { find: "@semio/js", replacement: path.resolve(__dirname, "../../js") },
        { find: "@semio/react", replacement: path.resolve(__dirname, "../../react") },
        // 🧷 Point directly at `semio.js` (the wasm-bindgen entry) so we don't depend on `pkg/package.json`,
        // which `wasm-pack build --no-pack` regenerates / wipes on every rebuild. Resilient to rebuilds.
        { find: "@semio/rs-wasm", replacement: path.resolve(__dirname, "../../rs/pkg/semio.js") },
        { find: "@semio/ui", replacement: path.resolve(__dirname, "../../../../../ui/react") },
        { find: "@ui/react", replacement: path.resolve(__dirname, "../../../../../ui/react") },
        { find: "@semio/sketchpad/shell", replacement: path.resolve(__dirname, "shell.tsx") },
        { find: "@semio/sketchpad", replacement: path.resolve(__dirname) },
        { find: "@semio/studio", replacement: path.resolve(__dirname, "../../studio") },
        { find: "@semio/assets/icons", replacement: path.resolve(__dirname, "../../../../assets/index.ts") },
        { find: "@semio/assets", replacement: path.resolve(__dirname, "../../../../assets") },
        { find: "@framework/core", replacement: path.resolve(__dirname, "../../../../../framework/core/index.ts") },
        { find: "@framework/platform/core", replacement: path.resolve(__dirname, "../../../../../framework/product/platform/core/index.ts") },
        { find: "@framework/platform/renderer/react", replacement: path.resolve(__dirname, "../../../../../framework/product/platform/renderer/react/index.tsx") },
        { find: "@framework/playground/core", replacement: path.resolve(__dirname, "../../../../../framework/product/playground/core/core.ts") },
        {
          find: "@framework/playground/renderer/react",
          replacement: path.resolve(__dirname, "../../../../../framework/product/playground/renderer/react/index.tsx"),
        },
        { find: "@puzzle/2d/react", replacement: path.resolve(__dirname, "../../../../../puzzle/2d/react/index.tsx") },
        { find: "@puzzle/3d/react", replacement: path.resolve(__dirname, "../../../../../puzzle/3d/react/index.tsx") },
        { find: "@puzzle/5d/react", replacement: path.resolve(__dirname, "../../../../../puzzle/5d/react/index.tsx") },
        { find: "@cad/js/renderer", replacement: path.resolve(__dirname, "../../../../../cad/js/renderer/index.tsx") },
        { find: /^@elements\/board$/, replacement: path.resolve(__dirname, "../../../../../elements/client/lib/board/index.ts") },
        { find: /^@elements\/scene$/, replacement: path.resolve(__dirname, "../../../../../elements/client/lib/scene/index.tsx") },
        { find: /^@elements\/topology$/, replacement: path.resolve(__dirname, "../../../../../elements/client/lib/topology/react/index.tsx") },
        { find: /^@elements\/ui-shell$/, replacement: path.resolve(__dirname, "../../../../../elements/core/index.ts") },
        { find: /^@elements\/ui$/, replacement: path.resolve(__dirname, "../../../../../elements/renderer/react/index.tsx") },
        { find: /^use-sync-external-store\/shim\/with-selector(?:\.js)?$/, replacement: shimWithSelector },
        { find: /^use-sync-external-store\/shim(?:\.js)?$/, replacement: shimMain },
        { find: "scheduler", replacement: schedulerEntry },
        { find: "vite/internal", replacement: viteInternalFallback },
      ],
    },
    plugins: [
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
      include: ["golden-layout", "scheduler", "use-sync-external-store/shim", "use-sync-external-store/shim/with-selector", "use-sync-external-store/with-selector"],
      exclude: ["@semio/js", "@semio/sketchpad", "@playwright/test", "playwright", "playwright-core"],
      esbuildOptions: {
        target: "es2020",
      },
    },
    server: {
      host: "0.0.0.0",
      port: 5173,
    },
    build: {
      /** Workers + wasm-bindgen glue may use syntax older `esbuild` targets cannot downlevel (see vite-plugin-top-level-await). */
      target: "es2022",
      rollupOptions: {
        external: ["@playwright/test", "node:fs/promises", "node:path", "node:url", "@semio/assets/semio/metabolism/wip/initialKit/kit.semio.json", "fs", "path", "url"],
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
