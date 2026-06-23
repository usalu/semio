// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build and development configuration for the playground app.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite build configuration for the play application.
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
import { playgroundIframeEmbedHeadersPlugin } from "../../../../../ui/styling/vite-elements-assets.ts";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
// #endregion 🔌Adapters

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

const PLAYWRIGHT_DEV_STUB_ID = "\0compose-sketchpad-play-playwright-dev-stub";
const EMBEDDED_NODE_TEST_REGIONS = [
  /\/\/#region 🧪Tests[\s\S]*?\/\/#endregion 🧪Tests\s*/,
  /\/\/#region 🧪E2E[\s\S]*?\/\/#endregion 🧪E2E\s*/,
];

function isEmbeddedNodeTestIndexModule(id: string): boolean {
  const file = id.replace(/\\/g, "/");
  return file.endsWith("/compose/client/lib/sketchpad/js/index.ts") || file.endsWith("/compose/client/lib/js/index.ts");
}

/** @emoji ✂️ Drops embedded vitest + Playwright regions from the browser bundle. */
function stripEmbeddedNodeTests(source: string): string {
  let next = source;
  for (const region of EMBEDDED_NODE_TEST_REGIONS) {
    next = next.replace(region, "");
  }
  return next;
}

/** @emoji ✂️ Drops embedded vitest + Playwright regions from the browser bundle. */
function stripEmbeddedNodeTestsPlugin(): Plugin {
  return {
    name: "compose-sketchpad-play-strip-embedded-node-tests",
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

/** @emoji 🧱 Keeps Playwright out of the browser graph when embedded E2E regions are scanned. */
function monorepoPlaywrightDevStubPlugin(): Plugin {
  return {
    name: "compose-sketchpad-play-playwright-dev-stub",
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

// Vite configuration with plugins, resolve aliases, and asset serving.
// Export MUST call defineConfig with the complete build configuration.
export default defineConfig(async () => {
  // 📥normal import fails in electron due to esm stuff
  const tailwind = await import("@tailwindcss/vite");
  const fs = await import("fs");
  const viteInternalFallback = path.resolve(__dirname, "../../../node_modules/vite/dist/node/index.js");
  return {
    base: "./",
    define: {
      __COMPOSE_JS_RUN_BENCHMARKS__: "false",
      __COMPOSE_JS_RUN_EMBEDDED_TESTS__: "false",
      __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
      "import.meta.env.COMPOSE_SKETCHPAD_E2E": JSON.stringify(process.env.COMPOSE_SKETCHPAD_E2E ?? ""),
    },
    resolve: {
      alias: [
        { find: "@semio-tech/compose-js", replacement: path.resolve(__dirname, "../../js") },
        { find: "@semio-tech/compose-rs-wasm", replacement: path.resolve(__dirname, "../../rs/pkg") },
        { find: "@compose/ui", replacement: path.resolve(__dirname, "../../../../../ui/react") },
        { find: "@semio-tech/compose-sketchpad", replacement: path.resolve(__dirname, "../react") },
        { find: "@compose/studio", replacement: path.resolve(__dirname, "../../studio") },
        { find: "@semio-tech/compose-asset", replacement: path.resolve(__dirname, "../../../../asset") },
        { find: /^@ui\/react$/, replacement: path.resolve(__dirname, "../../../../../ui/react/index.tsx") },
        { find: "vite/internal", replacement: viteInternalFallback },
      ],
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
      monorepoPlaywrightDevStubPlugin(),
      stripEmbeddedNodeTestsPlugin(),
      playgroundIframeEmbedHeadersPlugin(),
      wasm(),
      topLevelAwait(), // needed for older browsers to run wasm
      {
        name: "serve-wasm-and-assets",
        enforce: "pre" as const,
        configureServer(server: any) {
          const sketchpadPublicPath = path.resolve(__dirname, "../../sketchpad/public");
          const assetsPath = path.resolve(__dirname, "../../../../asset");
          const metabolismKitPath = path.resolve(assetsPath, "compose/metabolism.zip");
          server.middlewares.use((req: any, res: any, next: any) => {
            if (req.url?.endsWith(".wasm")) {
              const wasmFile = path.join(sketchpadPublicPath, req.url);
              if (fs.existsSync(wasmFile) && fs.statSync(wasmFile).isFile()) {
                res.setHeader("Content-Type", "application/wasm");
                fs.createReadStream(wasmFile).pipe(res);
                return;
              }
            }
            const bareUrl = req.url?.split(/[?#]/, 1)[0];
            if (bareUrl === "/metabolism.zip" && fs.existsSync(metabolismKitPath) && fs.statSync(metabolismKitPath).isFile()) {
              res.setHeader("Content-Type", "application/zip");
              fs.createReadStream(metabolismKitPath).pipe(res);
              return;
            }
            if (req.url?.startsWith("/asset/")) {
              const filePath = path.join(assetsPath, req.url.replace("/asset/", ""));
              if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
                fs.createReadStream(filePath).pipe(res);
                return;
              }
            }
            next();
          });
        },
        generateBundle(this: { emitFile: (asset: { type: "asset"; fileName: string; source: Buffer }) => void }) {
          const assetsPath = path.resolve(__dirname, "../../../../asset");
          const metabolismKitPath = path.resolve(assetsPath, "compose/metabolism.zip");
          if (fs.existsSync(metabolismKitPath) && fs.statSync(metabolismKitPath).isFile()) {
            this.emitFile({
              type: "asset",
              fileName: "metabolism.zip",
              source: fs.readFileSync(metabolismKitPath),
            });
          }
        },
      },
    ],
    optimizeDeps: {
      include: ["golden-layout"],
      exclude: ["@semio-tech/compose-js", "@semio-tech/compose-sketchpad", "@playwright/test", "playwright", "playwright-core"],
      esbuildOptions: {
        target: "es2020",
      },
    },
    ssr: {
      noExternal: ["golden-layout"],
    },
    build: {
      target: "es2022",
      rollupOptions: {
        external: ["@playwright/test"],
      },
    },
    worker: {
      format: "es",
    },
  };
});
// #endregion 🗄️Configuration
