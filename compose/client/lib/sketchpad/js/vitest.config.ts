// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  define: {
    __COMPOSE_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false",
  },
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-core": resolve(root, "../../../../../framework/core/index.ts"),
      "@semio-tech/framework-platform-core": resolve(root, "../../../../../framework/product/platform/core/index.ts"),
      "@semio-tech/compose-js": resolve(root, "../../js/index.ts"),
      "@semio-tech/reasoning-mindmap-wires-react": resolve(root, "../../../../../reasoning/mindmap/wires/react/index.ts"),
      "@semio-tech/reasoning-mindmap-react": resolve(root, "../../../../../reasoning/mindmap/react/index.tsx"),
      "@semio-tech/infinite-cavas-react-renderer": resolve(root, "../../../../../infinite/cavas/react-renderer/index.tsx"),
      "@semio-tech/infinite-world-r3f": resolve(root, "../../../../../infinite/world/r3f/index.tsx"),
      "@semio-tech/puzzle-2d-react": resolve(root, "../../../../../puzzle/2d/react/index.tsx"),
    },
  },
  test: {
    name: "@semio-tech/compose-sketchpad",
    environment: "node",
    include: ["index.ts"],
    coverage: { include: ["index.ts"] },
  },
});
