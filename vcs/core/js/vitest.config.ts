// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/vcs-core` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/vcs-core": resolve(root, "index.ts"),
      "@semio-tech/vcs-core/hasher": resolve(root, "hasher.ts"),
    },
  },
  test: {
    name: "@semio-tech/vcs-core",
    mode: "test",
    environment: "node",
    include: ["index.ts", "hasher.ts"],
    includeSource: ["index.ts", "hasher.ts"],
    passWithNoTests: false,
  },
});
