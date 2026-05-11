// This file has been automatically migrated to valid ESM format by Storybook.
// #region 🧲Header
// 💻 semio/ui/.storybook/main.ts
// Specs: Keep Storybook wiring aligned with elements/ui.
// Summary: Configures Storybook for the semio ui bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { StorybookConfig } from "@storybook/react-vite";
import { mergeConfig } from "vite";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join, resolve } from "path";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const repoRootPath = resolve(__dirname, "../../..");
const semioUiEntryPath = resolve(__dirname, "../index.tsx");
const elementsUiDir = resolve(__dirname, "../../../elements/ui");
const elementsUiEntryPath = resolve(elementsUiDir, "index.tsx");
const semioJsEntryPath = resolve(__dirname, "../../js/index.ts");
const semioRsWasmEntryPath = resolve(__dirname, "../../rs/pkg/semio.js");

function getAbsolutePath(value: string): string {
  try {
    return dirname(require.resolve(join(value, "package.json")));
  } catch {
    return dirname(require.resolve(join("../../../elements/ui/node_modules", value, "package.json")));
  }
}

const config: StorybookConfig = {
  stories: ["./stories/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"],
  addons: [getAbsolutePath("@storybook/addon-vitest"), getAbsolutePath("@storybook/addon-docs")],
  framework: {
    name: getAbsolutePath("@storybook/react-vite"),
    options: {},
  },
  docs: {},
  typescript: {
    reactDocgen: "react-docgen-typescript",
  },
  core: {
    disableTelemetry: true,
  },
  async viteFinal(config) {
    config.resolve = config.resolve || {};
    config.resolve.alias = {
      ...(config.resolve.alias || {}),
      "@semio/ui": semioUiEntryPath,
      "@elements/ui/elements": elementsUiEntryPath,
      "@elements/ui": elementsUiDir,
      "@semio/js": semioJsEntryPath,
      "@semio/rs-wasm": semioRsWasmEntryPath,
    };
    config.server = config.server || {};
    config.server.fs = {
      ...(config.server.fs || {}),
      allow: Array.from(new Set([...(config.server.fs?.allow || []), repoRootPath])),
    };
    const currentWatch = config.server.watch && typeof config.server.watch === "object" ? config.server.watch : {};
    const currentIgnored = currentWatch.ignored;
    const ignoredList = Array.isArray(currentIgnored) ? currentIgnored : (currentIgnored ? [currentIgnored] : []);
    config.server.watch = {
      ...currentWatch,
      usePolling: true,
      ignored: [
        ...ignoredList,
        "**/storybook-static/**",
        "**/.nx/**",
        "**/.repo/**",
        "**/dist/**",
        "**/.git/**",
        "**/node_modules/**",
      ],
    };

    config.plugins = config.plugins || [];
    const indicesToRemove: number[] = [];
    for (let i = 0; i < config.plugins.length; i++) {
      const plugin: any = config.plugins[i];
      if (plugin === "@mdx-js/rollup" || (plugin && typeof plugin === "object" && plugin.name === "@mdx-js/rollup")) {
        indicesToRemove.push(i);
        continue;
      }
      if (plugin instanceof Promise) {
        try {
          const resolved: any = await plugin;
          if (resolved && typeof resolved === "object" && resolved.name === "storybook:mdx-plugin") {
            indicesToRemove.push(i);
          }
        } catch (e) {}
      }
    }
    for (let i = indicesToRemove.length - 1; i >= 0; i--) {
      config.plugins.splice(indicesToRemove[i], 1);
    }

    const mdx = await import("@mdx-js/rollup");
    config.plugins.push(
      mdx.default({
        remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
        rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
      }),
    );

    config.optimizeDeps = config.optimizeDeps || {};
    config.optimizeDeps.include = [...(config.optimizeDeps.include || []), "golden-layout"];
    // @semio/js embeds Vitest suites behind `if (__vitest_worker__)` using top-level await; es2020
    // cannot parse that during dep-scan and Storybook then skips pre-bundling — dynamic story imports fail.
    config.optimizeDeps.esbuildOptions = {
      ...config.optimizeDeps.esbuildOptions,
      target: "es2022",
    };

    config.build = config.build || {};
    config.build.target = "es2022";

    config.mode = "development";
    config.define = {
      ...config.define,
      "process.env.NODE_ENV": JSON.stringify("development"),
    };

    // @semio/js uses `new Worker(..., { type: "module" })`; Vite defaults to worker.format=iife which
    // breaks Rollup when the worker graph is code-split. mergeConfig keeps Storybook's worker.plugins.
    return mergeConfig(config, {
      worker: {
        format: "es",
      },
    });
  },
};

export default config;
