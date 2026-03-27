// #region 🔖Header
// [👤semio📚engine💻vitemcpappconfig](repo://p/u/semio/b/l/engine/f/vite.mcp-app.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Bundles React, @semio/ui, and @modelcontextprotocol/ext-apps into one inlined HTML file.
// Summary: Vite build config bundling the MCP App into a single inlined HTML file.

// #endregion 🔖Header

import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import mdx from "@mdx-js/rollup";
import path from "path";

// #region 🔖ZodJitlessPlugin
// Specs: Zod v4 (dependency of @modelcontextprotocol/ext-apps) uses `new Function()`
// for JIT-compiled object parsing which violates CSP `script-src` in MCP App hosts.
// This plugin patches Zod's `allowsEval` check and `Doc.compile()` to never use
// dynamic code generation, forcing Zod to fall back to its interpreted parser.
// Summary: Vite plugin disabling Zod JIT to avoid CSP eval violations in MCP Apps.

function zodJitlessPlugin(): Plugin {
  return {
    name: "zod-jitless",
    enforce: "pre",
    transform(code, id) {
      if (!id.includes("zod")) return;
      if (id.endsWith("core/util.js") || id.endsWith("core/util.mjs")) {
        return code.replace(
          /export const allowsEval[\s\S]*?}\);/,
          "export const allowsEval = { get value() { return false; } };",
        );
      }
      if (id.endsWith("core/doc.js") || id.endsWith("core/doc.mjs")) {
        return code.replace(
          /compile\(\)\s*\{[\s\S]*?return new F\([^)]*\);\s*\}/,
          "compile() { return () => {}; }",
        );
      }
      return undefined;
    },
  };
}

// #endregion 🔖ZodJitlessPlugin

// #region 🔖StubHeavyDepsPlugin
// Specs: The MCP App only uses SemioDiagram (SVG) and SemioKit from @semio/ui.
// Three.js, cytoscape, sql.js, jszip, and other heavy deps are not needed.
// This plugin resolves them to empty stubs to keep the bundle small.
// Summary: Vite plugin stubbing heavy deps unused by the MCP App.

const STUB_EMPTY = path.resolve(__dirname, "stubs/empty.js");
const STUBBED_PREFIXES = ["three", "@react-three", "cytoscape", "sql.js", "jszip", "dagre", "fuse.js", "golden-layout", "@semio/js", "@semio/assets"];

function stubHeavyDepsPlugin(): Plugin {
  return {
    name: "stub-heavy-deps",
    enforce: "pre",
    resolveId(source) {
      for (const prefix of STUBBED_PREFIXES) {
        if (source === prefix || source.startsWith(prefix + "/")) return { id: "\0stub:" + source, syntheticNamedExports: true };
      }
      if (source === "i18next") return path.resolve(__dirname, "stubs/i18next.js");
      if (source === "i18next-browser-languagedetector") return { id: "\0stub:" + source, syntheticNamedExports: true };
      if (source === "react-i18next") return path.resolve(__dirname, "stubs/react-i18next.js");
      if (source === "react-router-dom") return path.resolve(__dirname, "stubs/react-router-dom.js");
      return null;
    },
    load(id) {
      if (id.startsWith("\0stub:")) return "const noop = () => noop; noop.prototype = {}; export default new Proxy(noop, { get: (_, p) => p === '__esModule' ? true : p === 'default' ? noop : noop });\n";
      return null;
    },
  };
}

// #endregion 🔖StubHeavyDepsPlugin

export default defineConfig({
  root: __dirname,
  build: {
    outDir: path.resolve(__dirname, "dist"),
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "mcp-app.html"),
      onwarn(warning, warn) {
        if (warning.code === "MISSING_EXPORT") return;
        warn(warning);
      },
    },
  },
  plugins: [stubHeavyDepsPlugin(), mdx(), zodJitlessPlugin(), viteSingleFile()],
  esbuild: {
    jsx: "automatic",
  },
});
