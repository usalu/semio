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
      "@semio-tech/cad-js-core": resolve(jsRoot, "core/index.ts"),
      "@semio-tech/cad-js-runtime": resolve(root, "index.ts"),
      "@semio-tech/cad-js-module-spatial-shape": resolve(jsRoot, "module/spatial-shape/index.ts"),
      "@semio-tech/cad-js-module-aec-building": resolve(jsRoot, "module/aec-building/index.ts"),
      "@semio-tech/cad-js-module-aec-building-energy": resolve(jsRoot, "module/aec-building-energy/index.ts"),
      "@semio-tech/cad-js-module-aec-building-structure": resolve(jsRoot, "module/aec-building-structure/index.ts"),
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
