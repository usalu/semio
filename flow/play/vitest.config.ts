// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for flow play playground wiring (`play/index.ts`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-playground-core": resolve(root, "../../framework/product/playground/core/index.ts"),
      "@semio-tech/framework-platform-core": resolve(root, "../../framework/product/platform/core/index.ts"),
      "@semio-tech/flow-react": resolve(root, "../react/index.tsx"),
      "@semio-tech/flow-module-math": resolve(root, "../module/math/pkg/flow_module_math.js"),
      "@semio-tech/flow-module-text": resolve(root, "../module/text/pkg/flow_module_text.js"),
      "@semio-tech/flow-module-logic": resolve(root, "../module/logic/pkg/flow_module_logic.js"),
      "@semio-tech/flow-module-dictionary": resolve(root, "../module/dictionary/pkg/flow_module_dictionary.js"),
      "@semio-tech/flow-module-list": resolve(root, "../module/list/pkg/flow_module_list.js"),
      "@semio-tech/ui-react": resolve(root, "../../ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
