// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/framework-core` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-core": resolve(root, "index.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-core",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
