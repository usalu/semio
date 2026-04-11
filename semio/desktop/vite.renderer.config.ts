// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron renderer process.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite configuration for the Electron renderer process with React and Tailwind.
// Configuration MUST enable the React and Tailwind CSS plugins.

import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
import mdx from "@mdx-js/rollup";
import remarkGfm from "remark-gfm";
import remarkFrontmatter from "remark-frontmatter";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import rehypeSlug from "rehype-slug";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

function useSyncExternalStoreShimResolvePlugin(shimMain: string, shimWithSelector: string): Plugin {
  return {
    name: "semio-use-sync-external-store-shim",
    enforce: "pre",
    resolveId(id) {
      const n = id.replace(/\\/g, "/");
      if (n.includes("use-sync-external-store/shim/with-selector")) {
        return shimWithSelector;
      }
      if (n.includes("use-sync-external-store/shim")) {
        return shimMain;
      }
      return undefined;
    },
  };
}

// Async Vite config loading Tailwind CSS, MDX, React and WASM plugins for the renderer.
// Export MUST return a valid Vite config with all plugins enabled.
export default defineConfig(async ({ mode }) => {
  const tailwind = await import("@tailwindcss/vite");
  const useSyncRoot = path.resolve(__dirname, "../../node_modules/use-sync-external-store/cjs");
  const prod = mode === "production";
  const shimMain = path.join(useSyncRoot, prod ? "use-sync-external-store-shim.production.js" : "use-sync-external-store-shim.development.js");
  const shimWithSelector = path.join(
    useSyncRoot,
    "use-sync-external-store-shim",
    prod ? "with-selector.production.js" : "with-selector.development.js",
  );
  return {
    server: {
      watch: {
        usePolling: true,
        interval: 1000,
      },
    },
    resolve: {
      dedupe: ["react", "react-dom", "use-sync-external-store"],
      // `shim/index.js` is CJS (`module.exports`); Vite would serve it as ESM and break `import { useSyncExternalStore }`.
      // Point bare specifiers at the CJS builds under `cjs/` so Rollup/commonjs rewrites exports (VS Code / zustand compatible).
      alias: [
        { find: /^use-sync-external-store\/shim\/with-selector(\.js)?$/, replacement: shimWithSelector },
        { find: /^use-sync-external-store\/shim(\/index\.js)?$/, replacement: shimMain },
        { find: "@semio/js", replacement: path.resolve(__dirname, "../js") },
        { find: "@semio/sketchpad", replacement: path.resolve(__dirname, "../sketchpad") },
        { find: "@semio/studio", replacement: path.resolve(__dirname, "../studio") },
        { find: "@semio/assets", replacement: path.resolve(__dirname, "../assets") },
      ],
    },
    plugins: [
      useSyncExternalStoreShimResolvePlugin(shimMain, shimWithSelector),
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
      topLevelAwait(),
    ],
    optimizeDeps: {
      entries: ["./renderer.tsx", "../sketchpad/index.tsx"],
      include: [
        "golden-layout",
        "@mdx-js/react",
        "use-sync-external-store/shim",
        "use-sync-external-store/shim/with-selector",
        "use-sync-external-store/with-selector",
      ],
      exclude: ["@semio/js", "@semio/sketchpad", "@semio/studio", "@playwright/test", "playwright", "playwright-core"],
    },
  };
});

// #endregion 🗄️Configuration
