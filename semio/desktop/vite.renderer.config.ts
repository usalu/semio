// #region 🔖Header
// [👤semio🖱️desktop⚙️viterendererconfig](repo://p/u/semio/b/u/desktop/f/vite.renderer.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron renderer process.

// #endregion 🔖Header

// #region 🔖Configuration
// [👤semio🖱️desktop⚙️viterendererconfig🔖configuration](repo://p/u/semio/b/u/desktop/f/vite.renderer.config.ts/s/Configuration)
// Vite configuration for the Electron renderer process with React and Tailwind.
// Configuration MUST enable the React and Tailwind CSS plugins.

import { defineConfig } from "vite";
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

// Async Vite config loading Tailwind CSS, MDX, React and WASM plugins for the renderer.
// Export MUST return a valid Vite config with all plugins enabled.
export default defineConfig(async () => {
  const tailwind = await import("@tailwindcss/vite");
  return {
    server: {
      watch: {
        usePolling: true,
        interval: 1000,
      },
    },
    resolve: {
      alias: {
        "@semio/js": path.resolve(__dirname, "../js"),
        "@semio/sketchpad": path.resolve(__dirname, "../sketchpad"),
        "@semio/studio": path.resolve(__dirname, "../studio"),
        "@semio/assets": path.resolve(__dirname, "../assets"),
      },
    },
    plugins: [
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
      entries: ["./renderer.tsx", "../sketchpad/index.ts"],
      include: ["golden-layout", "@mdx-js/react"],
      exclude: ["@semio/js", "@semio/sketchpad", "@semio/studio", "@playwright/test", "playwright", "playwright-core"],
    },
  };
});

// #endregion 🔖Configuration
