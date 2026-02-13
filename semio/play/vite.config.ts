// #region 🔖Header

// [⚙️semio/play/vite.config.ts](semiorepo://file/semio/play/vite.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build and development configuration for the playground app.

// #endregion 🔖Header

// #region 🔖Configuration
// Vite build configuration for the play application.
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
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

// Absolute file path of the current module.
// Path MUST be derived from import.meta.url.
const __filename = fileURLToPath(import.meta.url);
// Absolute directory path of the current module.
// Path MUST be derived from __filename.
const __dirname = path.dirname(__filename);

// Vite configuration with plugins, resolve aliases, and asset serving.
// Export MUST call defineConfig with the complete build configuration.
export default defineConfig(async () => {
  // normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite");
  const fs = await import("fs");
  return {
    resolve: {
      alias: {
        "@semio/js": path.resolve(__dirname, "../js"),
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
      topLevelAwait(), // needed for older browsers to run wasm
      {
        name: "serve-wasm-and-assets",
        enforce: "pre" as const,
        configureServer(server: any) {
          const jsPublicPath = path.resolve(__dirname, "../js/public");
          const assetsPath = path.resolve(__dirname, "../assets");
          server.middlewares.use((req: any, res: any, next: any) => {
            if (req.url?.endsWith(".wasm")) {
              const wasmFile = path.join(jsPublicPath, req.url);
              if (fs.existsSync(wasmFile) && fs.statSync(wasmFile).isFile()) {
                res.setHeader("Content-Type", "application/wasm");
                fs.createReadStream(wasmFile).pipe(res);
                return;
              }
            }
            if (req.url?.startsWith("/assets/")) {
              const filePath = path.join(assetsPath, req.url.replace("/assets/", ""));
              if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
                fs.createReadStream(filePath).pipe(res);
                return;
              }
            }
            next();
          });
        },
      },
    ],
    optimizeDeps: {
      include: ["golden-layout"],
      exclude: ["@semio/js"],
      esbuildOptions: {
        target: "es2020",
      },
    },
    ssr: {
      noExternal: ["golden-layout"],
    },
  };
});
// #endregion 🔖Configuration
