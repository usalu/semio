// #region 🔖Header
// [👤semio📚engine💻vitemcpappconfig](repo://p/u/semio/b/l/engine/f/vite.mcp-app.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Bundles React and @modelcontextprotocol/ext-apps only. No @semio/ui or heavy deps.
// Summary: Vite build config bundling the MCP App into a single inlined HTML file.

// #endregion 🔖Header

import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
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
      // Patch util.js: make allowsEval always return false
      if (id.endsWith("core/util.js") || id.endsWith("core/util.mjs")) {
        return code.replace(
          /export const allowsEval[\s\S]*?}\);/,
          "export const allowsEval = { get value() { return false; } };",
        );
      }
      // Patch doc.js: make compile() return a no-op function instead of using new Function
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

export default defineConfig({
  root: __dirname,
  build: {
    outDir: path.resolve(__dirname, "dist"),
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "mcp-app.html"),
    },
  },
  plugins: [zodJitlessPlugin(), viteSingleFile()],
  esbuild: {
    jsx: "automatic",
  },
});
