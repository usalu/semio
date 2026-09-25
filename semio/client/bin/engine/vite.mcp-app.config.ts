// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the standalone MCP App as a single HTML file.
// Reuses the sketchpad Vite config (workspace aliases, wasm, tailwind, MDX) and inlines everything with vite-plugin-singlefile.
// Summary: Vite build config bundling the @semio/sketchpad MCP App viewers into one inlined HTML file.

// #endregion 🧲Header

// #region 🔌Adapters
import path from "path";
import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import { sketchpadViteConfig } from "../../lib/sketchpad/js/vite.config.ts";
// #endregion 🔌Adapters

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

export default defineConfig(async ({ mode }) => {
  const base = await sketchpadViteConfig(mode);
  return {
    ...base,
    root: __dirname,
    plugins: [meshoptNoopPlugin(), zodJitlessPlugin(), ...(base.plugins ?? []), viteSingleFile()],
    build: {
      ...base.build,
      outDir: path.resolve(__dirname, "dist"),
      emptyOutDir: true,
      rollupOptions: {
        ...base.build?.rollupOptions,
        input: path.resolve(__dirname, "mcp-app.html"),
      },
    },
    worker: {
      format: "es",
      rollupOptions: {
        output: {
          inlineDynamicImports: true,
        },
      },
    },
  };
});
