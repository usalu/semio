// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the semio VS Code extension bundling.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite build configuration for the semio VS Code extension.
// Configuration MUST output a CJS bundle targeting Node 18.

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
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
// #endregion 🔌Adapters

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig(async ({ mode }) => {
  // Webview build: produces sketchpad-dist/ with an HTML app for the VS Code webview.
  if (mode === "webview") {
    const tailwind = await import("@tailwindcss/vite");
    return {
      root: __dirname,
      define: {
        __SEMIO_JS_RUN_BENCHMARKS__: "false",
        __SEMIO_JS_RUN_EMBEDDED_TESTS__: "false",
        __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
      },
      plugins: [
        // Stub Node.js-only modules, test frameworks, and benchmark assets as empty modules
        // so the browser bundle does not crash on missing Node.js built-ins.
        {
          name: "stub-node-modules",
          enforce: "pre" as const,
          resolveId(source: string) {
            const nodeBuiltins = ["fs", "path", "url", "child_process", "crypto", "os", "stream", "util", "events", "buffer", "http", "https", "net", "tls", "zlib", "assert", "querystring", "string_decoder", "tty", "worker_threads"];
            const stubTargets = ["@playwright/test", "better-sqlite3", "electron"];
            if (source.startsWith("node:") || nodeBuiltins.includes(source) || stubTargets.includes(source)) {
              return `\0stub-node:${source}`;
            }
            // Stub benchmark/test JSON assets — use .js virtual ID to avoid vite:json plugin.
            if (/kit_metabolism|metabolism\.kit|kit_metabolism_diffed|diff_kit_metabolism|kit_invalid/.test(source)) {
              return `\0stub-asset:${source.replace(/\.json$/, "")}.js`;
            }
            return null;
          },
          load(id: string) {
            if (id.startsWith("\0stub-node:") || id.startsWith("\0stub-asset:")) {
              return "export default {};";
            }
            return null;
          },
        },
        tailwind.default(),
        {
          ...mdx({
            remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
            rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
            providerImportSource: "@mdx-js/react",
          }),
          enforce: "pre" as const,
        },
        react(),
        wasm(),
        topLevelAwait(),
      ],
      build: {
        outDir: "sketchpad-dist",
        emptyOutDir: false,
        rollupOptions: {
          input: path.resolve(__dirname, "webview.html"),
        },
        target: "esnext",
        sourcemap: true,
        minify: false,
      },
      resolve: {
        alias: {
          "@semio/sketchpad": path.resolve(__dirname, "../sketchpad"),
          "@semio/js": path.resolve(__dirname, "../js"),
          "@elements/ui": path.resolve(__dirname, "../../elements/ui"),
          "@semio/assets": path.resolve(__dirname, "../assets"),
        },
      },
    };
  }

  // Extension host build: produces out/extension.js targeting Node 18.
  return {
    build: {
      lib: {
        entry: path.resolve(__dirname, "extension.ts"),
        formats: ["cjs"],
        fileName: () => "extension",
      },
      rollupOptions: {
        external: ["vscode"],
        output: {
          entryFileNames: "extension.js",
          format: "cjs",
          sourcemap: true,
        },
      },
      outDir: "out",
      emptyOutDir: true,
      minify: false,
      sourcemap: true,
      target: "node18",
      ssr: true,
    },
    ssr: {
      noExternal: true,
    },
  };
});
// #endregion 🗄️Configuration
