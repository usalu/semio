// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "../../..");

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/cad-js-core": resolve(jsRoot, "core/js/index.ts"),
      "@semio-tech/cad-js-runtime": resolve(jsRoot, "runtime/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building": resolve(root, "index.ts"),
    },
  },
  test: {
    name: "@semio-tech/cad-js-module-aec-building",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
    includeSource: ["index.ts"],
  },
});
