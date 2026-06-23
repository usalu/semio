// #region 🧲Header
/** @emoji 🧪 Vitest for `@semio-tech/reasoning-mindmap-wires-play`. */
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
      "@semio-tech/framework-playground-core": path.resolve(repoRoot, "framework/product/playground/core/index.ts"),
      "@semio-tech/framework-platform-core": path.resolve(repoRoot, "framework/product/platform/core/index.ts"),
      "@semio-tech/infinite-cavas-react-renderer": path.resolve(repoRoot, "infinite/cavas/react-renderer/index.tsx"),
      "@semio-tech/puzzle-2d-play": path.resolve(repoRoot, "puzzle/2d/play/index.ts"),
      "@semio-tech/puzzle-2d-react": path.resolve(repoRoot, "puzzle/2d/react/index.tsx"),
      "@semio-tech/reasoning-mindmap-wires-react": path.resolve(playDir, "../react/index.ts"),
      "@semio-tech/ui-react": path.resolve(repoRoot, "ui/react/index.tsx"),
    },
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    passWithNoTests: false,
  },
});
