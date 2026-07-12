// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/framework-os-core` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os-core": resolve(root, "index.ts"),
    },
  },
  test: {
    mode: "test",
    environment: "node",
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
