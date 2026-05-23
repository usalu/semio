// #region 🧲Header
// 💻 semio/dev/algorithms/vitest.config.ts
// Specs: Vitest for @semio/algorithms index (embedded tests + WASM-backed runner smoke).
// Summary: Mirrors semio/js wasm resolution; `@semio/assets` alias for fixture imports in tests.
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const semioWasmBg = path.resolve(__dirname, "../../client/lib/rs/pkg/semio_bg.wasm");
const semioRsPkg = path.resolve(__dirname, "../../client/lib/rs/pkg/semio.js");
const semioAssets = path.resolve(__dirname, "../../assets");

export default defineConfig({
  resolve: {
    alias: {
      "@semio/rs-wasm": semioRsPkg,
      "@semio/assets": semioAssets,
    },
  },
  test: {
    name: "@semio/algorithms",
    environment: "node",
    globals: true,
    testTimeout: 120_000,
    include: ["index.ts"],
    passWithNoTests: false,
    exclude: ["**/node_modules/**", "**/dist/**", "**/.storybook/**"],
    env: { SEMIO_WASM_BG_PATH: semioWasmBg },
  },
});
