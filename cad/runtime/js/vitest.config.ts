// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "..");

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/cad-js-core": resolve(root, "../../core/js/index.ts"),
      "@semio-tech/cad-js-runtime": resolve(root, "index.ts"),
      "@semio-tech/cad-js-module-spatial-shape": resolve(root, "../../module/spatial-shape/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building": resolve(root, "../../module/aec-building/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building-energy": resolve(root, "../../module/aec-building-energy/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building-structure": resolve(root, "../../module/aec-building-structure/js/index.ts"),
    },
  },
  test: {
    name: "@semio-tech/cad-js-runtime",
    mode: "test",
    environment: "node",
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.ts"],
    includeSource: ["index.ts"],
  },
});
