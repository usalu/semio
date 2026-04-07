// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Bundles React, @semio/ui, and @modelcontextprotocol/ext-apps into one inlined HTML file.
// Summary: Vite build config bundling the MCP App into a single inlined HTML file.

// #endregion 🧲Header

import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import path from "path";
import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";

// #region 🪵ZodJitlessPlugin
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
        return code.replace(/export const allowsEval[\s\S]*?}\);/, "export const allowsEval = { get value() { return false; } };");
      }
      if (id.endsWith("core/doc.js") || id.endsWith("core/doc.mjs")) {
        return code.replace(/compile\(\)\s*\{[\s\S]*?return new F\([^)]*\);\s*\}/, "compile() { return () => {}; }");
      }
      return undefined;
    },
  };
}

// #endregion 🪵ZodJitlessPlugin

// #region 🎺StubHeavyDepsPlugin
// Specs: semio/ui/index.tsx imports 3D and semio-assets modules at module scope.
// Stubbing those modules keeps the MCP App bundle small, but the stubs must be JSON-safe
// (so Vite doesn't try to parse stubbed `*.json` as real JSON).
// Summary: Stub heavy deps + semio assets (JSON-safe) to keep the MCP App fast and reliable.

// NOTE: Do NOT stub `three` / `@react-three/*`.
// `semio/ui/index.tsx` imports named exports at module scope and an ESM stub
// that doesn't provide those exact exports can crash the bundle before React mounts.
// We only stub semio assets and unrelated heavy deps.
// cytoscape is NOT stubbed: flattenDesign uses cytoscape headless BFS for 2D layout (diagram centers + planes).
// 🔧Stubbing it breaks the diagram entirely because cy.elements() returns undefined → TypeError.
const STUBBED_PREFIXES = ["@semio/assets", "sql.js", "jszip", "dagre", "fuse.js", "golden-layout"];

// #region 🧱MeshoptNoopPlugin
// Specs: three-stdlib/libs/MeshoptDecoder calls WebAssembly.instantiate() in an IIFE at module
// scope. The MCP App host iframe CSP blocks wasm-eval, causing a rejection that can crash the
// scene. This plugin patches the resolved MeshoptDecoder file to return {supported:false}
// immediately, avoiding any WebAssembly usage while keeping the export shape intact.
// Summary: Vite plugin neutralizing MeshoptDecoder WASM to avoid CSP wasm-eval violations.

function meshoptNoopPlugin(): Plugin {
  const MESHOPT_STUB = `
const noop = () => {};
const MeshoptDecoder = { supported: false, ready: Promise.resolve(), decode: noop, decodeGltfBuffer: noop };
export { MeshoptDecoder };
export default MeshoptDecoder;
`;
  return {
    name: "meshopt-noop",
    enforce: "pre",
    load(id) {
      if ((id.includes("MeshoptDecoder") || id.includes("meshopt_decoder")) && !id.includes("node_modules/.cache")) {
        return MESHOPT_STUB;
      }
      return null;
    },
  };
}

// #endregion 🧱MeshoptNoopPlugin

function stubHeavyDepsPlugin(): Plugin {
  return {
    name: "stub-heavy-deps",
    enforce: "pre",
    resolveId(source) {
      for (const prefix of STUBBED_PREFIXES) {
        if (source === prefix || source.startsWith(prefix + "/")) {
          // Vite applies its JSON plugin based on file extension; ensure stub ids don't end with ".json".
          if (source.endsWith(".json")) {
            return {
              id: "\0stub-json:" + source.slice(0, -5),
              syntheticNamedExports: true,
            };
          }
          return { id: "\0stub:" + source, syntheticNamedExports: true };
        }
      }
      if (source === "i18next") return path.resolve(__dirname, "stubs/i18next.js");
      if (source === "i18next-browser-languagedetector") return { id: "\0stub:" + source, syntheticNamedExports: true };
      if (source === "react-i18next") return path.resolve(__dirname, "stubs/react-i18next.js");
      if (source === "react-router-dom") return path.resolve(__dirname, "stubs/react-router-dom.js");
      return null;
    },
    load(id) {
      if (id.startsWith("\0stub-json:")) return "export default {};\n";
      if (id.startsWith("\0stub:")) return "const noop = () => noop; noop.prototype = {}; export default new Proxy(noop, { get: (_, p) => (p === '__esModule' ? true : p === 'default' ? noop : noop) });\n";
      return null;
    },
  };
}

// #endregion 🎺StubHeavyDepsPlugin

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
  plugins: [meshoptNoopPlugin(), stubHeavyDepsPlugin(), tailwindcss(), mdx(), zodJitlessPlugin(), viteSingleFile()],
  esbuild: {
    jsx: "automatic",
  },
});
