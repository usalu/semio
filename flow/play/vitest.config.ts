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
      "@framework/playground/core": resolve(root, "../../framework/product/playground/core/index.ts"),
      "@framework/platform/core": resolve(root, "../../framework/product/platform/core/index.ts"),
      "@flow/react": resolve(root, "../react/index.tsx"),
      "@flow/module-math": resolve(root, "../modules/math/pkg/flow_module_math.js"),
      "@flow/module-text": resolve(root, "../modules/text/pkg/flow_module_text.js"),
      "@flow/module-logic": resolve(root, "../modules/logic/pkg/flow_module_logic.js"),
      "@flow/module-dictionary": resolve(root, "../modules/dictionary/pkg/flow_module_dictionary.js"),
      "@flow/module-list": resolve(root, "../modules/list/pkg/flow_module_list.js"),
      "@ui/react": resolve(root, "../../ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
