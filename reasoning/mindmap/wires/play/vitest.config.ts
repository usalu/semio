// #region 🧲Header
/** @emoji 🧪 Vitest for `@reasoning/mindmap/wires/play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");

export default defineConfig({
  root: playDir,
  resolve: {
    alias: {
      "@framework/playground/core": path.resolve(repoRoot, "framework/product/playground/core/index.ts"),
      "@framework/platform/core": path.resolve(repoRoot, "framework/product/platform/core/index.ts"),
      "@infinite/cavas/react-renderer": path.resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx"),
      "@puzzle/2d/play": path.resolve(repoRoot, "puzzle/2d/play/index.ts"),
      "@puzzle/2d/react": path.resolve(repoRoot, "puzzle/2d/react/index.tsx"),
      "@reasoning/mindmap/wires/react": path.resolve(playDir, "../react/index.ts"),
      "@ui/react": path.resolve(repoRoot, "ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    passWithNoTests: false,
  },
});
