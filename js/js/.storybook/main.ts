// #region Header

// main.ts

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

import type { StorybookConfig } from "@storybook/react-vite";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import remarkGfm from "remark-gfm";

import { dirname, join } from "path";

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 */
function getAbsolutePath(value: string): any {
  return dirname(require.resolve(join(value, "package.json")));
}
const config: StorybookConfig = {
  stories: ["../**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"],
  addons: [
    // getAbsolutePath('@storybook/addon-essentials'),
    // getAbsolutePath('@chromatic-com/storybook'),
    getAbsolutePath("@storybook/addon-vitest"),
    getAbsolutePath("@storybook/addon-docs"),
  ],

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
    config.plugins = config.plugins || [];

    // Find and remove all MDX-related plugins (both @mdx-js/rollup and storybook:mdx-plugin)
    const indicesToRemove: number[] = [];

    for (let i = 0; i < config.plugins.length; i++) {
      const plugin: any = config.plugins[i];

      // Check if it's a direct MDX plugin reference
      if (plugin === '@mdx-js/rollup' || (plugin && typeof plugin === 'object' && plugin.name === '@mdx-js/rollup')) {
        indicesToRemove.push(i);
        continue;
      }

      // Check if it's a Promise that contains storybook:mdx-plugin
      if (plugin instanceof Promise) {
        try {
          const resolved: any = await plugin;
          if (resolved && typeof resolved === 'object' && resolved.name === 'storybook:mdx-plugin') {
            indicesToRemove.push(i);
          }
        } catch (e) {
          // Ignore promise resolution errors
        }
      }
    }

    // Remove all MDX plugins in reverse order
    for (let i = indicesToRemove.length - 1; i >= 0; i--) {
      config.plugins.splice(indicesToRemove[i], 1);
    }

    // Add our single configured MDX plugin
    const mdx = await import("@mdx-js/rollup");
    config.plugins.push(
      mdx.default({
        remarkPlugins: [remarkGfm, remarkFrontmatter, remarkMdxFrontmatter],
        rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
      })
    );

    return config;
  },
};
export default config;
