// #region 🧲Header
// 💻 compose/dev/algorithm/vitest.config.ts
// Specs: Vitest for @semio-tech/compose-algorithm index (embedded tests + WASM-backed runner smoke).
// Summary: Mirrors compose/js wasm resolution; `@semio-tech/compose-asset` and `@semio-tech/compose-fixture` aliases.
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const composeWasmBg = path.resolve(__dirname, "../../client/lib/rs/pkg/compose_bg.wasm");
const composeRsPkg = path.resolve(__dirname, "../../client/lib/rs/pkg/compose.js");
const composeAssets = path.resolve(__dirname, "../../asset");
const composeFixtures = path.resolve(__dirname, "../../fixture");

export default defineConfig({
  resolve: {
    alias: {
      "@semio-tech/compose-rs-wasm": composeRsPkg,
      "@semio-tech/compose-asset": composeAssets,
      "@semio-tech/compose-fixture": composeFixtures,
    },
  },
  test: {
    name: "@semio-tech/compose-algorithm",
    environment: "node",
    globals: true,
    testTimeout: 120_000,
    include: ["index.ts"],
    passWithNoTests: false,
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
    env: { COMPOSE_WASM_BG_PATH: composeWasmBg },
  },
});
