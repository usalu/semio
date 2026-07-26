// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/ui-styling` (inline `import.meta.vitest` in `vite-elements-assets.ts`:
 * `tileProxyVitePlugin`/`staticDirVitePlugin`/`meshCollectionVitePlugin`/`playgroundAssetVitePlugins` and
 * every other in-source `describe` block in that file). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/ui-styling": resolve(root, "js/index.ts"),
    },
  },
  test: {
    name: "@semio-tech/ui-styling",
    mode: "test",
    environment: "node",
    include: ["vite-elements-assets.ts"],
    coverage: { include: ["vite-elements-assets.ts"] },
    includeSource: ["vite-elements-assets.ts"],
    passWithNoTests: false,
  },
});
