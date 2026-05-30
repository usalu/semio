// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/playground/renderer/react`. */
export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@framework/playground", replacement: resolve(root, "../../core/core.ts") },
      { find: "@framework/playground/renderer/react/shell", replacement: resolve(root, "shell.tsx") },
      { find: "@framework/playground/renderer/react/boot", replacement: resolve(root, "shell.tsx") },
      { find: "@framework/playground/renderer/react/puzzle/2d", replacement: resolve(root, "index.tsx") },
      { find: "@framework/playground/renderer/react/puzzle/3d", replacement: resolve(root, "index.tsx") },
      { find: "@framework/playground/renderer/react/puzzle/5d", replacement: resolve(root, "index.tsx") },
      { find: "@framework/playground/renderer/react", replacement: resolve(root, "index.tsx") },
      { find: "@ui/react", replacement: resolve(root, "../../../../ui/react/index.tsx") },
      { find: "@puzzle/2d/play", replacement: resolve(root, "../../../../puzzle/2d/play/index.ts") },
      { find: "@puzzle/3d/play", replacement: resolve(root, "../../../../puzzle/3d/play/index.ts") },
      { find: "@puzzle/5d/play", replacement: resolve(root, "../../../../puzzle/5d/play/index.ts") },
      { find: "@puzzle/2d/react", replacement: resolve(root, "../../../../puzzle/2d/react/index.tsx") },
      { find: "@puzzle/3d/react", replacement: resolve(root, "../../../../puzzle/3d/react/index.tsx") },
      { find: "@puzzle/5d/react", replacement: resolve(root, "../../../../puzzle/5d/react/index.tsx") },
    ],
  },
  test: {
    environment: "jsdom",
    include: ["shell.tsx"],
    passWithNoTests: true,
  },
});
