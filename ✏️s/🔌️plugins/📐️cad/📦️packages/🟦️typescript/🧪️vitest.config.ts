// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { createWorkspaceViteResolveConfig } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const root = resolve(configDir, "../.."); // ✏️s/🔌️plugins/📐️cad
const repoRoot = resolve(configDir, "../../../../..");
const reactRoot = resolve(repoRoot, "node_modules/react");
const reactDomRoot = resolve(repoRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "node_modules/three");

const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot);

const DOMAIN_FILES = ["🔨️modules/🫀️core/🟦️component.ts", "🔨️modules/📺️renderer/🟦️component.tsx", "🔨️modules/📐️brepjs/🟦️component.ts", "🔨️modules/🔍️query/🟦️component.ts", "🔨️modules/🎰️stately/🟦️component.ts", "🔨️modules/🏃️runtime/🟦️component.ts"];

/** @emoji 🧪️ Vitest for `@semio-tech/cad-js` — one project covering all 6 folded domain files (former cad-js-{core,renderer,kernel-brepjs,query,machine-stately,runtime} configs merged; renderer alone needs jsdom, the rest run in `node`). */
export default defineConfig({
  root,
  plugins: [],
  assetsInclude: ["**/*.wasm"],
  server: workspaceResolve.server,
  resolve: {
    alias: [...(workspaceResolve.resolve?.alias ?? []), { find: /^react$/, replacement: resolve(reactRoot, "index.js") }, { find: /^react\/jsx-runtime$/, replacement: resolve(reactRoot, "jsx-runtime.js") }, { find: /^react\/jsx-dev-runtime$/, replacement: resolve(reactRoot, "jsx-dev-runtime.js") }, { find: /^react-dom$/, replacement: resolve(reactDomRoot, "index.js") }, { find: /^react-dom\/client$/, replacement: resolve(reactDomRoot, "client.js") }, { find: /^three$/, replacement: threeModule }, { find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` }],
  },
  test: {
    name: "@semio-tech/cad-js",
    mode: "test",
    include: DOMAIN_FILES,
    includeSource: DOMAIN_FILES,
    coverage: { include: DOMAIN_FILES },
    environment: "node",
    environmentMatchGlobs: [["🔨️modules/📺️renderer/🟦️component.tsx", "jsdom"]],
    passWithNoTests: false,
    server: {
      deps: {
        inline: [/@semio-tech\/.*/, /cad\/.*/],
      },
    },
  },
});
