// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "../..");

/** @emoji 🧪️ Vitest for `@semio-tech/cad-js-query` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/cad-js-core": resolve(jsRoot, "core/js/index.ts"),
      "@semio-tech/cad-js-runtime": resolve(jsRoot, "runtime/js/index.ts"),
      "@semio-tech/cad-js-module-spatial-shape": resolve(jsRoot, "module/spatial-shape/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building": resolve(jsRoot, "module/aec-building/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building-energy": resolve(jsRoot, "module/aec-building-energy/js/index.ts"),
      "@semio-tech/cad-js-module-aec-building-structure": resolve(jsRoot, "module/aec-building-structure/js/index.ts"),
      "@semio-tech/cad-js-kernel-brepjs": resolve(jsRoot, "kernel/brepjs/js/index.ts"),
    },
  },
  test: {
    name: "@semio-tech/cad-js-query",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
