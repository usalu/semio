// #region Header

// vite.config.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import tailwindcss from "@tailwindcss/vite";
import mdx from "@mdx-js/rollup";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  plugins: [
    tailwindcss(),
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
    include: ["golden-layout", "three"],
    esbuildOptions: {
      target: "es2020",
    },
  },
  resolve: {
    dedupe: ["three"],
    alias: {
      "@semio/assets": path.resolve(__dirname, "../../assets"),
    },
  },
  ssr: {
    noExternal: ["golden-layout"],
  },
  test: {
    environment: "node",
    testTimeout: 30000,
    include: ["**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      exclude: ["**/*.config.*", "**/*.setup.*", "**/node_modules/**", "**/.storybook/**"],
    },
  },
});
