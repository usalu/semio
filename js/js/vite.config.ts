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
//vitest.config.ts
//2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Lesser General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Lesser General Public License for more details.

//You should have received a copy of the GNU Lesser General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

import mdx from "@mdx-js/rollup";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import path from "node:path";
import { fileURLToPath } from "node:url";

import mdx from "@mdx-js/rollup";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { defineConfig } from "vitest/config";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig(async () => {
  const tailwind = await import("@tailwindcss/vite");
  const isStorybookTest = process.env.VITEST_PROJECT === "storybook";
  let storybookPlugin: any = undefined;
  if (isStorybookTest) {
    const { storybookTest } = await import("@storybook/addon-vitest/vitest-plugin");
    storybookPlugin = storybookTest({ configDir: path.join(__dirname, ".storybook") });
  }
  return {
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
      ...(storybookPlugin ? [storybookPlugin] : []),
    ],
    optimizeDeps: {
      include: ["golden-layout", "three"],
      esbuildOptions: {
        target: "es2020",
      },
    },
    resolve: {
      dedupe: ["three"],
    },
    ssr: {
      noExternal: ["golden-layout"],
    },
    test: {
      globals: true,
      projects: [
        {
          name: "unit",
          test: {
            environment: "node",
            include: ["**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
            exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
          },
        },
        ...(isStorybookTest
          ? [
              {
                name: "storybook",
                test: {
                  browser: {
                    enabled: true,
                    headless: true,
                    name: "chromium",
                    provider: "playwright",
                  },
                  setupFiles: [".storybook/vitest.setup.ts"],
                },
              },
            ]
          : []),
      ],
      coverage: {
        provider: "v8",
        reporter: ["text", "json", "html"],
        exclude: ["**/*.config.*", "**/*.setup.*", "**/node_modules/**", "**/.storybook/**"],
      },
    },
  };
});
