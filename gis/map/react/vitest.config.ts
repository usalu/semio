// #region 🧲Header
/** @emoji 🧪 Vitest for `@semio-tech/gis-map-react`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const reactDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  test: {
    environment: "node",
    include: ["index.tsx"],
  },
  resolve: {
    alias: {
      "@semio-tech/gis-map-rs": path.resolve(reactDir, "../rs/pkg/gis_map.js"),
    },
  },
});
