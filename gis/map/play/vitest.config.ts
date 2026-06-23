// #region 🧲Header
/** @emoji 🧪 Vitest for `@semio-tech/gis-map-play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: playDir,
  resolve: {
    alias: {
      "@semio-tech/framework-playground-core": path.resolve(playDir, "../../../framework/product/playground/core/index.ts"),
      "@semio-tech/framework-platform-core": path.resolve(playDir, "../../../framework/product/platform/core/index.ts"),
      "@semio-tech/gis-map-react": path.resolve(playDir, "../react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    passWithNoTests: false,
  },
});
