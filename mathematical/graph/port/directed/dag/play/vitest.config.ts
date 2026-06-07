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
      "@framework/playground/core": resolve(repoRoot, "framework/product/playground/core/index.ts"),
      "@framework/platform/core": resolve(repoRoot, "framework/product/platform/core/index.ts"),
      "@framework/playground/renderer/react/dag": resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx"),
      "@dag/react": resolve(root, "../react/index.tsx"),
      "@ui/react": resolve(repoRoot, "ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
