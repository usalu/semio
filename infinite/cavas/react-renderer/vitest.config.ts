// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/infinite-cavas-react-renderer`. */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../ui/js/react/index.tsx") }],
  },
  test: {
    mode: "test",
    environment: "jsdom",
    include: ["index.tsx"],
    includeSource: ["index.tsx"],
    passWithNoTests: true,
  },
});
