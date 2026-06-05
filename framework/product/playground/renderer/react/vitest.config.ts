// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { playgroundRendererResolveAliases, playgroundRendererShellEntryPlugin, playgroundRendererVitestShellOnlyPlugin } from "../../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../");
const rendererIndex = resolve(root, "index.tsx");

/** @emoji 🧪 Vitest for `@framework/playground/renderer/react`. */
export default defineConfig({
  root,
  plugins: [react(), playgroundRendererVitestShellOnlyPlugin(rendererIndex), playgroundRendererShellEntryPlugin(rendererIndex)],
  resolve: {
    alias: [
      { find: /^@framework\/playground\/core$/, replacement: resolve(root, "../../core/index.ts") },
      { find: /^@framework\/playground\/renderer\/react$/, replacement: resolve(root, "index.tsx") },
      ...playgroundRendererResolveAliases(repoRoot),
    ],
  },
  test: {
    environment: "jsdom",
    include: ["index.tsx"],
    passWithNoTests: true,
    env: {
      PLAYGROUND_RENDERER_SHELL_ONLY: "1",
    },
  },
});
