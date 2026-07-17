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
      "@semio-tech/cad-js-core": resolve(jsRoot, "core/index.ts"),
      "@semio-tech/cad-js-runtime": resolve(jsRoot, "runtime/index.ts"),
      "@semio-tech/cad-js-kernel-brepjs": resolve(jsRoot, "kernel/brepjs/index.ts"),
      "@semio-tech/cad-js-module-aec-building-energy": resolve(root, "index.ts"),
    },
  },
  test: {
    name: "@semio-tech/cad-js-module-aec-building-energy",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
  },
});
