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
      { find: "@flow/module-core", replacement: resolve(root, "../module/core/pkg/flow_module_core.js") },
      { find: "@flow/module-math", replacement: resolve(root, "../module/math/pkg/flow_module_math.js") },
      { find: "@flow/module-text", replacement: resolve(root, "../module/text/pkg/flow_module_text.js") },
      { find: "@flow/module-logic", replacement: resolve(root, "../module/logic/pkg/flow_module_logic.js") },
      { find: "@flow/module-dictionary", replacement: resolve(root, "../module/dictionary/pkg/flow_module_dictionary.js") },
      { find: "@flow/module-list", replacement: resolve(root, "../module/list/pkg/flow_module_list.js") },
      { find: "@flow/module-brep", replacement: resolve(root, "../module/brep/pkg/flow_module_brep.js") },
      { find: "@flow/module-bim", replacement: resolve(root, "../module/bim/pkg/flow_module_bim.js") },
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
