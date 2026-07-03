// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { createWorkspaceViteResolveConfig, playgroundRendererVitestShellOnlyPlugin } from "../../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../");
const rendererIndex = resolve(root, "index.tsx");

const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

/** @emoji 🧪 Vitest for `@semio-tech/framework-playground-renderer-react`. */
export default defineConfig({
  root,
  plugins: [react(), playgroundRendererVitestShellOnlyPlugin(rendererIndex)],
  resolve: {
    alias: [
      { find: /^@framework\/playground\/core$/, replacement: resolve(root, "../../core/index.ts") },
      { find: /^@framework\/playground\/renderer\/react$/, replacement: resolve(root, "index.tsx") },
      ...(workspaceResolve.resolve?.alias ?? []),
    ],
    dedupe: workspaceResolve.resolve?.dedupe,
  },
  server: workspaceResolve.server,
  optimizeDeps: workspaceResolve.optimizeDeps,
  test: {
    environment: "jsdom",
    include: ["index.tsx"],
    passWithNoTests: true,
    env: {
      PLAYGROUND_RENDERER_SHELL_ONLY: "1",
    },
  },
});
