// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { uiAssetsVitePlugin } from "../../../../framework/ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../../");
const uiAssetsRoot = resolve(repoRoot, "framework/ui/asset");

/** @emoji 🧪 Vitest for `@semio-tech/mit-bestand-praesentation-projektetage`. */
export default defineConfig({
  root: dir,
  plugins: [...uiAssetsVitePlugin(uiAssetsRoot), tailwindcss(), react()],
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "framework/ui/js/react/index.tsx") },
      { find: "@semio-tech/animate-present-core", replacement: resolve(repoRoot, "animate/present/core/js/index.ts") },
      { find: "@semio-tech/animate-present-renderer-react", replacement: resolve(repoRoot, "animate/present/renderer/react/index.tsx") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "framework/core/js/index.ts") },
      {
        find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
        replacement: resolve(dir, "index.ts"),
      },
    ],
  },
  test: {
    name: "@semio-tech/mit-bestand-praesentation-projektetage",
    mode: "test",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
    includeSource: ["index.ts"],
    passWithNoTests: false,
  },
});
