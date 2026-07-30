// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron renderer process.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite configuration for the Electron renderer process with React and Tailwind.
// Configuration MUST enable the React and Tailwind CSS plugins.

// #region 🔌Adapters
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
import { createWorkspaceViteResolveConfig, semioFaviconVitePlugin } from "../../../../../framework/module/ui/styling/🟦vite-elements-assets.ts";
// #endregion 🔌Adapters

type CjsFacadeResolveOpts = {
  htmlParseStringifyEntry: string;
  reactRouterEntry: string;
  reactI18nextEntry: string;
  shimMain: string;
  shimWithSelector: string;
  schedulerEntry: string;
  statsEntry: string;
};

function reactCjsFacadeResolvePlugin(opts: CjsFacadeResolveOpts): Plugin {
  const cookieFacadeId = "\0compose-cjs-facade:cookie";
  const voidElementsFacadeId = "\0compose-cjs-facade:void-elements";

  return {
    name: "compose-react-cjs-facades",
    enforce: "pre",
    resolveId(id) {
      const n = id.replace(/\\/g, "/");
      if (n === "cookie") {
        return cookieFacadeId;
      }
      if (n === "void-elements") {
        return voidElementsFacadeId;
      }
      if (n.includes("use-sync-external-store/shim/with-selector")) {
        return opts.shimWithSelector;
      }
      if (n.includes("use-sync-external-store/shim")) {
        return opts.shimMain;
      }
      // `scheduler/index.js` is a CJS re-export (`module.exports = require('./cjs/...')`); Vite `/@fs/...` breaks default import.
      if (n === "scheduler" || (n.includes("scheduler/index.js") && !n.includes("/cjs/scheduler."))) {
        return opts.schedulerEntry;
      }
      // `stats.js` ships as UMD/CJS; Vite may serve `/@fs/.../build/stats.min.js` directly and break default import.
      // Force the ESM-compatible implementation from `three/examples` so drei `Stats` can load in Electron dev.
      if (n === "stats.js" || n.endsWith("/stats.js")) {
        return opts.statsEntry;
      }
      if (n === "html-parse-stringify" || n.endsWith("/html-parse-stringify")) {
        return opts.htmlParseStringifyEntry;
      }
      if (n === "react-i18next" || n.endsWith("/react-i18next")) {
        return opts.reactI18nextEntry;
      }
      if (n === "react-router" || n.endsWith("/react-router")) {
        return opts.reactRouterEntry;
      }
      return undefined;
    },
    load(id) {
      if (id === cookieFacadeId) {
        return `
const cookieModule = {
  parse(input = "") {
    const result = Object.create(null);
    for (const chunk of String(input).split(";")) {
      const index = chunk.indexOf("=");
      if (index < 0) {
        continue;
      }
      const key = chunk.slice(0, index).trim();
      if (!key || key in result) {
        continue;
      }
      const value = chunk.slice(index + 1).trim();
      try {
        result[key] = decodeURIComponent(value);
      } catch {
        result[key] = value;
      }
    }
    return result;
  },
  serialize(name, value, options = {}) {
    const encodedValue = options.encode ? options.encode(String(value)) : encodeURIComponent(String(value));
    const segments = [\`\${name}=\${encodedValue}\`];
    if (options.maxAge !== undefined) {
      segments.push(\`Max-Age=\${Math.floor(options.maxAge)}\`);
    }
    if (options.domain) {
      segments.push(\`Domain=\${options.domain}\`);
    }
    if (options.path) {
      segments.push(\`Path=\${options.path}\`);
    }
    if (options.expires instanceof Date) {
      segments.push(\`Expires=\${options.expires.toUTCString()}\`);
    }
    if (options.httpOnly) {
      segments.push("HttpOnly");
    }
    if (options.secure) {
      segments.push("Secure");
    }
    if (options.partitioned) {
      segments.push("Partitioned");
    }
    if (options.priority) {
      segments.push(\`Priority=\${options.priority}\`);
    }
    if (options.sameSite) {
      const sameSite = typeof options.sameSite === "string" ? options.sameSite : options.sameSite === true ? "Strict" : "";
      if (sameSite) {
        segments.push(\`SameSite=\${sameSite}\`);
      }
    }
    return segments.join("; ");
  },
  parseCookie(input, options) {
    return cookieModule.parse(input, options);
  },
  stringifyCookie(record, options = {}) {
    return Object.entries(record).map(([name, value]) => cookieModule.serialize(name, value, options)).join("; ");
  },
  stringifySetCookie(record, options = {}) {
    return cookieModule.stringifyCookie(record, options);
  },
  parseSetCookie(input) {
    return cookieModule.parse(input);
  },
};
export const parse = cookieModule.parse;
export const serialize = cookieModule.serialize;
export const parseCookie = cookieModule.parseCookie;
export const stringifyCookie = cookieModule.stringifyCookie;
export const stringifySetCookie = cookieModule.stringifySetCookie;
export const parseSetCookie = cookieModule.parseSetCookie;
export default cookieModule;
`;
      }
      if (id === voidElementsFacadeId) {
        return `
const voidElementsModule = {
  area: true,
  base: true,
  br: true,
  col: true,
  embed: true,
  hr: true,
  img: true,
  input: true,
  link: true,
  meta: true,
  param: true,
  source: true,
  track: true,
  wbr: true,
};
export default voidElementsModule;
`;
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
  const shimWithSelector = path.join(useSyncRoot, "use-sync-external-store-shim", prod ? "with-selector.production.js" : "with-selector.development.js");
  const schedulerRoot = path.resolve(__dirname, "../../node_modules/scheduler/cjs");
  const schedulerEntry = path.join(schedulerRoot, prod ? "scheduler.production.js" : "scheduler.development.js");
  const statsEntry = path.resolve(__dirname, "../../node_modules/three/examples/jsm/libs/stats.module.js");
  const htmlParseStringifyEntry = path.resolve(__dirname, "../../node_modules/html-parse-stringify/dist/html-parse-stringify.js");
  const reactI18nextEntry = path.resolve(__dirname, "../../node_modules/react-i18next/dist/commonjs/index.js");
  const reactRouterEntry = path.resolve(__dirname, "../../node_modules/react-router/dist/development/index.js");
  const repoRoot = path.resolve(__dirname, "../../../../..");
  const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot, [
    { find: /^use-sync-external-store\/shim\/with-selector(\.js)?$/, replacement: shimWithSelector },
    { find: /^use-sync-external-store\/shim(\/index\.js)?$/, replacement: shimMain },
    { find: /^scheduler$/, replacement: schedulerEntry },
    { find: /^html-parse-stringify$/, replacement: htmlParseStringifyEntry },
    { find: /^react-i18next$/, replacement: reactI18nextEntry },
    { find: /^react-router$/, replacement: reactRouterEntry },
    { find: /^stats\.js$/, replacement: statsEntry },
  ]);
  return {
    server: {
      ...workspaceResolve.server,
      watch: {
        usePolling: true,
        interval: 1000,
      },
    },
    define: {
      __COMPOSE_JS_RUN_BENCHMARKS__: "false",
      __COMPOSE_JS_RUN_EMBEDDED_TESTS__: "false",
      __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
    },
    resolve: {
      ...workspaceResolve.resolve,
      dedupe: [
        "three",
        "cookie",
        "dagre",
        "graphlib",
        "html-parse-stringify",
        "lodash",
        "react",
        "react-dom",
        "react-i18next",
        "react-router",
        "scheduler",
        "stats.js",
        "use-sync-external-store",
        "void-elements",
        "@react-three/fiber",
        "@react-three/drei",
        ...(workspaceResolve.resolve?.dedupe ?? []),
      ],
      alias: workspaceResolve.resolve?.alias,
    },
    optimizeDeps: workspaceResolve.optimizeDeps,
    plugins: [
      ...semioFaviconVitePlugin(repoRoot),
      reactCjsFacadeResolvePlugin({ htmlParseStringifyEntry, reactI18nextEntry, reactRouterEntry, shimMain, shimWithSelector, schedulerEntry, statsEntry }),
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
        "cookie",
        "dagre",
        "graphlib",
        "html-parse-stringify",
        "lodash",
        "react-i18next",
        "react-router",
        "scheduler",
        "stats.js",
        "use-sync-external-store/shim",
        "use-sync-external-store/shim/with-selector",
        "use-sync-external-store/with-selector",
        "void-elements",
      ],
      exclude: ["@semio-tech/compose-js", "@semio-tech/compose-sketchpad", "@playwright/test", "playwright", "playwright-core"],
    },
  };
});

// #endregion 🗄️Configuration
