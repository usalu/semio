// #region Header
// 2026 Ueli Saluz <ueli@semio-tech.de>
// Specs: Vite single-file MCP App bundle for coda (@semio-tech/ui-react + ext-apps).
// Summary: Mirrors compose/engine/vite.mcp-app.config.ts with @semio-tech/ui-react and meshopt noop.
// #endregion Header

// #region 🔌Adapters
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import path from "path";
import { defineConfig, type Plugin } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
// #endregion 🔌Adapters

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

const STUBBED_PREFIXES = ["@semio-tech/semio-asset", "sql.js", "jszip", "dagre", "fuse.js", "golden-layout"];

function stubHeavyDepsPlugin(): Plugin {
  return {
    name: "stub-heavy-deps",
    enforce: "pre",
    resolveId(source) {
      for (const prefix of STUBBED_PREFIXES) {
        if (source === prefix || source.startsWith(prefix + "/")) {
          if (source.endsWith(".json")) {
            return { id: "\0stub-json:" + source.slice(0, -5), syntheticNamedExports: true };
          }
          return { id: "\0stub:" + source, syntheticNamedExports: true };
        }
      }
      if (source === "i18next") return path.resolve(__dirname, "../../compose/engine/stubs/i18next.js");
      if (source === "i18next-browser-languagedetector") return { id: "\0stub:" + source, syntheticNamedExports: true };
      if (source === "react-i18next") return path.resolve(__dirname, "../../compose/engine/stubs/react-i18next.js");
      if (source === "react-router-dom") return path.resolve(__dirname, "../../compose/engine/stubs/react-router-dom.js");
      return null;
    },
    load(id) {
      if (id.startsWith("\0stub-json:")) return "export default {};\n";
      if (id.startsWith("\0stub:")) return "const noop = () => noop; noop.prototype = {}; export default new Proxy(noop, { get: (_, p) => (p === '__esModule' ? true : p === 'default' ? noop : noop) });\n";
      return null;
    },
  };
}

export default defineConfig({
  root: __dirname,
  resolve: {
    alias: {
      "@semio-tech/ui-react": path.resolve(__dirname, "../../../ui/react"),
    },
  },
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
