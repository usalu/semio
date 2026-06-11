// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@flow/play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig, FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "flow",
  extraAliases: [
    { find: "@flow/react", replacement: path.resolve(playDir, "../react/index.tsx") },
    { find: "@flow/module-core", replacement: path.resolve(playDir, "../module/core/pkg/flow_module_core.js") },
  ],
  resolveDedupe: ["react", "react-dom", "@flow/react"],
  optimizeDeps: {
    include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@flow/react"],
    exclude: [...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: [
    "../core/lib.rs",
    "../core/target/**",
    "../core/Cargo.toml",
    "../core/Cargo.lock",
    "../core/script.ts",
    "../module/**/lib.rs",
    "../module/**/target/**",
    "../module/**/Cargo.toml",
    "../module/**/script.ts",
  ],
});
