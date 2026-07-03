// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { createWorkspaceViteResolveConfig } from "../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../..");
const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

/** @emoji 🧪 Vitest for `@semio-tech/flow-react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
  root,
  resolve: workspaceResolve.resolve,
  server: workspaceResolve.server,
  optimizeDeps: workspaceResolve.optimizeDeps,
  test: {
    mode: "test",
    environment: "jsdom",
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.tsx"],
    includeSource: ["index.tsx"],
    passWithNoTests: true,
  },
});
