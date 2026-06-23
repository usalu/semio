// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../");

/** @emoji 🧪 Vitest for dag play playground wiring (`play/index.ts`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-playground-core": resolve(repoRoot, "framework/product/playground/core/index.ts"),
      "@semio-tech/framework-platform-core": resolve(repoRoot, "framework/product/platform/core/index.ts"),
      "@semio-tech/framework-playground-renderer-react/dag": resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx"),
      "@semio-tech/dag-react": resolve(root, "../react/index.tsx"),
      "@semio-tech/ui-react": resolve(repoRoot, "ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
