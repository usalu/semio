// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Bundles React, @semio-tech/ui-react, and @representationcontextprotocol/ext-apps into one inlined HTML file.
// Summary: Vite build config bundling the MCP App into a single inlined HTML file.

// #endregion 🧲️Header

// #region 🔌️Adapters
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import path from "path";
import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import { createWorkspaceViteResolveConfig } from "../../../../../framework/module/ui/styling/🟦️vite-elements-assets.ts";
// #endregion 🔌️Adapters

const repoRoot = path.resolve(__dirname, "../../../../../");
const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

// #region 🪵️ZodJitlessPlugin
// Specs: Zod v4 (dependency of @representationcontextprotocol/ext-apps) uses `new Function()`
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

// #endregion 🪵️ZodJitlessPlugin

// #region 🎺️StubHeavyDepsPlugin
// Specs: compose/ui/index.tsx imports 3D and semio-assets modules at module scope.
// Stubbing those modules keeps the MCP App bundle small, but the stubs must be JSON-safe
// (so Vite doesn't try to parse stubbed `*.json` as real JSON).
// Summary: Stub heavy deps + semio assets (JSON-safe) to keep the MCP App fast and reliable.

// NOTE: Do NOT stub `three` / `@react-three/*`.
// `compose/ui/index.tsx` imports named exports at module scope and an ESM stub
// that doesn't provide those exact exports can crash the bundle before React mounts.
// We only stub semio assets and unrelated heavy deps.
// flattenDesign uses native adjacency BFS (no cytoscape) for 2D layout (diagram centers + planes).
// 🔧️Stubbing it breaks the diagram entirely because cy.elements() returns undefined → TypeError.
const STUBBED_PREFIXES = ["@semio-tech/semio-asset", "sql.js", "jszip", "dagre", "fuse.js", "golden-layout"];

// #region 🧱️MeshoptNoopPlugin
// Specs: three-stdlib/libs/MeshoptDecoder calls WebAssembly.instantiate() in an IIFE at module
// scope. The MCP App host iframe CSP blocks wasm-eval, causing a rejection that can crash the
// scene. This plugin patches the resolved MeshoptDecoder file to return {supported:false}
// immediately, avoiding any WebAssembly usage while keeping the export shape intact.
// Summary: Vite plugin neutralizing MeshoptDecoder WASM to avoid CSP wasm-eval violations.

function meshoptNoopPlugin(): Plugin {
  const MESHOPT_STUB = `
const no_operation = () => {};
const MeshoptDecoder = { supported: false, ready: Promise.resolve(), decode: no_operation, decodeGltfBuffer: no_operation };
export { MeshoptDecoder };
export default MeshoptDecoder;
`;
  return {
    name: "meshopt-no-operation",
    enforce: "pre",
    load(id) {
      if ((id.includes("MeshoptDecoder") || id.includes("meshopt_decoder")) && !id.includes("node_modules/.cache")) {
        return MESHOPT_STUB;
      }
      return null;
    },
  };
}

// #endregion 🧱️MeshoptNoopPlugin

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
      if (id.startsWith("\0stub:")) return "const no_operation = () => no_operation; no_operation.prototype = {}; export default new Proxy(no_operation, { get: (_, p) => (p === '__esModule' ? true : p === 'default' ? no_operation : no_operation) });\n";
      return null;
    },
  };
}

// #endregion 🎺️StubHeavyDepsPlugin

export default defineConfig({
  root: __dirname,
  define: {
    __SEMIO_JS_RUN_BENCHMARKS__: "false",
    __SEMIO_JS_RUN_EMBEDDED_TESTS__: "false",
  },
  resolve: workspaceResolve.resolve,
  server: workspaceResolve.server,
  optimizeDeps: workspaceResolve.optimizeDeps,
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
  worker: {
    format: "es",
    rollupOptions: {
      output: {
        inlineDynamicImports: true,
      },
    },
  },
  esbuild: {
    jsx: "automatic",
  },
});
