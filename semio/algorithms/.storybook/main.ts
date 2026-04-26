// This file has been automatically migrated to valid ESM format by Storybook.
// #region 🧲Header
// 💻 semio/algorithms/.storybook/main.ts
// Specs: Keep Storybook wiring aligned with .elements/ui.
// Summary: Configures Storybook for the algorithms bundle.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { StorybookConfig } from "@storybook/react-vite";
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
const elementsUiDir = resolve(__dirname, "../../../elements/ui");
const elementsUiEntryPath = resolve(elementsUiDir, "index.tsx");
const algorithmsEntryPath = resolve(__dirname, "../index.ts");
const semioRsWasmPath = resolve(__dirname, "../../rs/pkg");
const semioUiEntryPath = resolve(__dirname, "../../ui/index.tsx");
const semioJsEntryPath = resolve(__dirname, "../../js/index.ts");

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
      "@elements/ui/elements": elementsUiEntryPath,
      "@elements/ui": elementsUiDir,
      "@semio/ui": semioUiEntryPath,
      "@semio/js": semioJsEntryPath,
      "@semio/rs-wasm": semioRsWasmPath,
      "@semio/assets": resolve(__dirname, "../../assets"),
      "@semio/algorithms": algorithmsEntryPath,
    };
    config.assetsInclude = [...(config.assetsInclude ?? []), "**/*.wasm"];
    config.server = config.server || {};
    config.server.proxy = config.server.proxy || {};
    config.server.fs = {
      ...(config.server.fs || {}),
      allow: Array.from(new Set([...(config.server.fs?.allow || []), repoRootPath])),
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
    config.optimizeDeps.exclude = Array.from(new Set([...(config.optimizeDeps.exclude || []), "@semio/ui", "@semio/js", "@semio/assets", "@elements/ui", "@elements/ui/elements"]));
    config.optimizeDeps.esbuildOptions = {
      ...config.optimizeDeps.esbuildOptions,
      target: "es2020",
    };

    config.mode = "development";
    config.define = {
      ...config.define,
      "process.env.NODE_ENV": JSON.stringify("development"),
    };

    // ⚙️Storybook builds use code-splitting; module-worker (semio/js spawns `./worker.ts` with `{type:"module"}`) requires ES worker format (iife rejects split chunks).
    config.worker = {
      ...(config.worker || {}),
      format: "es",
    };

    return config;
  },
};

export default config;
