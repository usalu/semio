// #region 🔖Header
// [👤semio📚engine💻vitemcpappconfig](repo://p/u/semio/b/l/engine/f/vite.mcp-app.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Bundles React, @semio/ui, and @modelcontextprotocol/ext-apps into one inlined HTML file.
// MUST exclude test deps (vitest, playwright), test assets (kit_metabolism), and sketchpad.
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

// #region 🔖StripTestAndSketchpadPlugin
// Specs: Strip test-only code blocks, test dependencies, test assets, and sketchpad
// imports from the production MCP App bundle. Resolves them to empty modules.
// Summary: Vite plugin that stubs out test/sketchpad/playwright imports for production builds.

const BLOCKED_PATTERNS = [
  /\/sketchpad\//,
  /playwright/,
  /vitest/,
  /kit_metabolism\.json/,
  /\.test\./,
  /\.spec\./,
];

function stripTestAndSketchpadPlugin(): Plugin {
  return {
    name: "strip-test-and-sketchpad",
    enforce: "pre",
    resolveId(source, importer) {
      for (const pattern of BLOCKED_PATTERNS) {
        if (pattern.test(source)) return { id: "\0empty-module", moduleSideEffects: false };
      }
      return null;
    },
    load(id) {
      if (id === "\0empty-module") return "export default {};";
      return null;
    },
    transform(code, id) {
      if (!id.includes("semio/js/index") && !id.includes("semio/ui/index") && !id.includes("elements/ui/index")) return;
      return code
        .replace(/if\s*\(\s*typeof\s*\(globalThis\s+as\s+any\)\.__vitest_worker__\s*!==\s*"undefined"\s*\)\s*\{[\s\S]*$/m, "")
        .replace(/if\s*\(\s*import\.meta\.vitest\s*\)\s*\{[\s\S]*?^}/m, "")
        .replace(/const\s+\w+\s*=\s*import\.meta\.vitest;\s*if\s*\(\s*\w+\s*\)\s*\{[\s\S]*?^}/gm, "")
        .replace(/const\s+\w+\s*=\s*\(\s*import\.meta[\s\S]*?\.vitest[\s\S]*?\)\.vitest;\s*if\s*\(\s*\w+\s*\)\s*\{[\s\S]*?^}/gm, "");
    },
  };
}

// #endregion 🔖StripTestAndSketchpadPlugin

export default defineConfig({
  root: __dirname,
  build: {
    outDir: path.resolve(__dirname, "dist"),
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "mcp-app.html"),
    },
  },
  plugins: [stripTestAndSketchpadPlugin(), mdx(), zodJitlessPlugin(), viteSingleFile()],
  esbuild: {
    jsx: "automatic",
  },
});
