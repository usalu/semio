// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { createWorkspaceViteResolveConfig } from "../../../../../framework/ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../..");
const reactRoot = resolve(repoRoot, "node_modules/react");
const reactDomRoot = resolve(repoRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "node_modules/three");

const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

export default defineConfig({
  root,
  plugins: [],
  assetsInclude: ["**/*.wasm"],
  server: workspaceResolve.server,
  resolve: {
    alias: [
      ...(workspaceResolve.resolve?.alias ?? []),
      { find: "@semio-tech/cad-js-core", replacement: resolve(root, "../../core/js/index.ts") },
      { find: "@semio-tech/cad-js-kernel-brepjs", replacement: resolve(root, "../../kernel/brepjs/js/index.ts") },
      { find: "@semio-tech/cad-js-machine-stately", replacement: resolve(root, "../../machine/stately/js/index.ts") },
      { find: "@semio-tech/cad-js-query", replacement: resolve(root, "../../query/js/index.ts") },
      { find: "@semio-tech/cad-js-runtime", replacement: resolve(root, "../../runtime/js/index.ts") },
      { find: "@semio-tech/cad-js-module-spatial-shape", replacement: resolve(root, "../../module/spatial-shape/js/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building", replacement: resolve(root, "../../module/aec-building/js/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building-energy", replacement: resolve(root, "../../module/aec-building-energy/js/index.ts") },
      { find: "@semio-tech/cad-js-module-aec-building-structure", replacement: resolve(root, "../../module/aec-building-structure/js/index.ts") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "framework/os/kernel/infinite/world/r3f/index.tsx") },
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
    name: "@semio-tech/cad-js-renderer",
    mode: "test",
    environment: "jsdom",
    include: ["index.tsx", "play/index.tsx", "play/fixture-slugs.ts"],
    coverage: { include: ["index.tsx", "play/index.tsx", "play/fixture-slugs.ts"] },
    server: {
      deps: {
        inline: [/@semio-tech\/.*/, /cad\/.*/],
      },
    },
  },
});
