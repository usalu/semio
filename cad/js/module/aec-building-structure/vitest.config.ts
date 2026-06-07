// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "../..");

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@cad/js/core": resolve(jsRoot, "core/index.ts"),
      "@cad/js/runtime": resolve(jsRoot, "runtime/index.ts"),
      "@cad/js/kernel/brepjs": resolve(jsRoot, "kernel/brepjs/index.ts"),
      "@cad/js/module/aec-building": resolve(jsRoot, "module/aec-building/index.ts"),
      "@cad/js/module/aec-building-structure": resolve(root, "index.ts"),
    },
  },
  test: {
    mode: "test",
    environment: "node",
    testTimeout: 120_000,
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.ts"],
    includeSource: ["index.ts"],
  },
});
