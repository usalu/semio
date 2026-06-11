// #region 🧲Header
// 💻 semio/dev/algorithm/vitest.config.ts
// Specs: Vitest for @semio/algorithm index (embedded tests + WASM-backed runner smoke).
// Summary: Mirrors semio/js wasm resolution; `@semio/asset` and `@semio/fixture` aliases.
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const semioWasmBg = path.resolve(__dirname, "../../client/lib/rs/pkg/semio_bg.wasm");
const semioRsPkg = path.resolve(__dirname, "../../client/lib/rs/pkg/semio.js");
const semioAssets = path.resolve(__dirname, "../../asset");
const semioFixtures = path.resolve(__dirname, "../../fixture");

export default defineConfig({
  resolve: {
    alias: {
      "@semio/rs-wasm": semioRsPkg,
      "@semio/asset": semioAssets,
      "@semio/fixture": semioFixtures,
    },
  },
  test: {
    name: "@semio/algorithm",
    environment: "node",
    globals: true,
    testTimeout: 120_000,
    include: ["index.ts"],
    passWithNoTests: false,
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
    env: { SEMIO_WASM_BG_PATH: semioWasmBg },
  },
});
