// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/animate-present-core`. */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/animate-present-core": resolve(root, "index.ts"),
      "@semio-tech/vcs-core": resolve(root, "../../../../vcs/core/js/index.ts"),
    },
  },
  test: {
    name: "@semio-tech/animate-present-core",
    mode: "test",
    environment: "node",
    include: ["index.ts", "internal.ts"],
    includeSource: ["index.ts", "internal.ts"],
    passWithNoTests: false,
  },
});
