// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/animate-present-core`. */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/animate-present-core": resolve(root, "index.ts"),
    },
  },
  test: {
    name: "@semio-tech/animate-present-core",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
