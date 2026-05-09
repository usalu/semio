// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build and development configuration for the sketchpad app.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite build configuration for the sketchpad application.
// Configuration MUST include MDX, React, WASM, and Tailwind CSS plugins.

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

type CjsFacadeResolveOpts = {
  shimMain: string;
  shimWithSelector: string;
  schedulerEntry: string;
};

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
  const assetsPath = path.resolve(__dirname, "../assets");
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
  const prod = mode === "production";
  const useSyncRoot = path.resolve(__dirname, "../../node_modules/use-sync-external-store/cjs");
  const shimMain = path.join(useSyncRoot, prod ? "use-sync-external-store-shim.production.js" : "use-sync-external-store-shim.development.js");
  const shimWithSelector = path.join(useSyncRoot, "use-sync-external-store-shim", prod ? "with-selector.production.js" : "with-selector.development.js");
  const schedulerRoot = path.resolve(__dirname, "../../node_modules/scheduler/cjs");
  const schedulerEntry = path.join(schedulerRoot, prod ? "scheduler.production.js" : "scheduler.development.js");
  const viteInternalFallback = path.resolve(__dirname, "../../node_modules/vite/dist/node/index.js");
  return {
    define: {
      __SEMIO_JS_RUN_BENCHMARKS__: "false",
      __SEMIO_JS_RUN_EMBEDDED_TESTS__: "false",
      __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
    },
    resolve: {
      dedupe: ["react", "react-dom", "scheduler", "use-sync-external-store"],
      alias: [
        { find: "@semio/js", replacement: path.resolve(__dirname, "../js") },
        // 🧷 Point directly at `semio.js` (the wasm-bindgen entry) so we don't depend on `pkg/package.json`,
        // which `wasm-pack build --no-pack` regenerates / wipes on every rebuild. Resilient to rebuilds.
        { find: "@semio/rs-wasm", replacement: path.resolve(__dirname, "../rs/pkg/semio.js") },
        { find: "@semio/ui", replacement: path.resolve(__dirname, "../ui") },
        { find: "@semio/sketchpad", replacement: path.resolve(__dirname) },
        { find: "@semio/studio", replacement: path.resolve(__dirname, "../studio") },
        { find: "@semio/assets", replacement: path.resolve(__dirname, "../assets") },
        { find: /^@elements\/ui\/elements$/, replacement: path.resolve(__dirname, "../../elements/ui/index.tsx") },
        { find: /^@elements\/ui$/, replacement: path.resolve(__dirname, "../../elements/ui/index.tsx") },
        { find: /^use-sync-external-store\/shim\/with-selector(?:\.js)?$/, replacement: shimWithSelector },
        { find: /^use-sync-external-store\/shim(?:\.js)?$/, replacement: shimMain },
        { find: "scheduler", replacement: schedulerEntry },
        { find: "vite/internal", replacement: viteInternalFallback },
      ],
    },
    plugins: [
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
      react(),
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
        external: ["@playwright/test", "node:fs/promises", "node:path", "node:url", "@semio/assets/semio/metabolism.kit.semio.json", "fs", "path", "url"],
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
