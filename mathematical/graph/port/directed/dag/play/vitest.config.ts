// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@dag/play` inlined source tests. */
export default defineConfig({
  root,
  test: {
    mode: "test",
    environment: "jsdom",
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: true,
  },
  resolve: {
    alias: [
      { find: "@dag/react", replacement: resolve(root, "../react/index.tsx") },
      { find: "@framework/playground/core", replacement: resolve(root, "../../../../framework/product/playground/core/index.ts") },
    ],
  },
});
