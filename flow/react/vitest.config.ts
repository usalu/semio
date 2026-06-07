// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@flow/react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@infinite/cavas/react-renderer", replacement: resolve(root, "../../infinite/cavas/react-renderer/index.tsx") },
      { find: "@flow/core", replacement: resolve(root, "../core/pkg/flow_core.js") },
    ],
  },
  test: {
    mode: "test",
    environment: "jsdom",
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.tsx"],
    includeSource: ["index.tsx"],
    passWithNoTests: true,
  },
});
