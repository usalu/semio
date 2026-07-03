// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { createWorkspaceViteResolveConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../..");
const reactRoot = resolve(repoRoot, "node_modules/react");
const reactDomRoot = resolve(repoRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "node_modules/three");
const rendererRoot = resolve(repoRoot, "framework/product/playground/renderer/react");
const rendererIndex = resolve(rendererRoot, "index.tsx");

const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

export default defineConfig({
  root,
  plugins: [],
  assetsInclude: ["**/*.wasm"],
  server: workspaceResolve.server,
  resolve: {
    alias: [
      { find: /^@framework\/playground\/renderer\/react($|\/.*$)/, replacement: rendererIndex },
      ...(workspaceResolve.resolve?.alias ?? []),
      { find: "@semio-tech/cad-js-core", replacement: resolve(root, "../core/index.ts") },
      { find: "@semio-tech/cad-js-kernel-brepjs", replacement: resolve(root, "../kernel/brepjs/index.ts") },
      { find: "@semio-tech/cad-js-machine-stately", replacement: resolve(root, "../machine/stately/index.ts") },
      { find: "@semio-tech/cad-js-query", replacement: resolve(root, "../query/index.ts") },
      { find: "@semio-tech/cad-js-runtime", replacement: resolve(root, "../runtime/index.ts") },
      { find: "@semio-tech/cad-js-module-spatial-shape", replacement: resolve(root, "../module/spatial-shape/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building", replacement: resolve(root, "../module/aec-building/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building-energy", replacement: resolve(root, "../module/aec-building-energy/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building-structure", replacement: resolve(root, "../module/aec-building-structure/index.ts") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "infinite/world/r3f/index.tsx") },
      { find: /^react$/, replacement: resolve(reactRoot, "index.js") },
      { find: /^react\/jsx-runtime$/, replacement: resolve(reactRoot, "jsx-runtime.js") },
      { find: /^react\/jsx-dev-runtime$/, replacement: resolve(reactRoot, "jsx-dev-runtime.js") },
      { find: /^react-dom$/, replacement: resolve(reactDomRoot, "index.js") },
      { find: /^react-dom\/client$/, replacement: resolve(reactDomRoot, "client.js") },
      { find: /^three$/, replacement: threeModule },
      { find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
    ],
  },
  test: {
    mode: "test",
    environment: "jsdom",
    testTimeout: 120_000,
    fileParallelism: false,
    maxConcurrency: 1,
    include: ["index.tsx", "play/index.tsx", "play/fixture-slugs.ts"],
  },
});
