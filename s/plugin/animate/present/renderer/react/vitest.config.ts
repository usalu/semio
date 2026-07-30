// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../..");

/** @emoji 🧪 Vitest for `@semio-tech/animate-present-renderer-react`. */
export default defineConfig({
  root,
  plugins: [react()],
  resolve: {
    alias: [
      { find: "@semio-tech/animate-present-core", replacement: resolve(root, "../../core/js/index.ts") },
      { find: "@semio-tech/animate-present-renderer-react", replacement: resolve(root, "index.tsx") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "framework/core/js/index.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "framework/ui/js/react/index.tsx") },
      {
        find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
        replacement: resolve(repoRoot, "mit-bestand/präsentation/33.projektetage/js/index.ts"),
      },
    ],
  },
  test: {
    name: "@semio-tech/animate-present-renderer-react",
    mode: "test",
    environment: "jsdom",
    include: ["index.tsx"],
    coverage: { include: ["index.tsx"] },
    includeSource: ["index.tsx"],
    passWithNoTests: false,
    setupFiles: ["./vitest.setup.ts"],
  },
});
