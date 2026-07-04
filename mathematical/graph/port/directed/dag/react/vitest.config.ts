// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/dag-react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/dag-core", replacement: resolve(root, "../rs/pkg/mathematical_graph_port_directed_dag.js") }],
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
