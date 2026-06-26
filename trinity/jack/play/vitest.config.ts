import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../..");

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-playground-core": resolve(repoRoot, "framework/product/playground/core/index.ts"),
      "@semio-tech/framework-platform-core": resolve(repoRoot, "framework/product/platform/core/index.ts"),
      "@semio-tech/trinity-react": resolve(repoRoot, "trinity/react/index.tsx"),
      "@semio-tech/framework-playground-renderer-react/trinity-jack": resolve(repoRoot, "framework/product/playground/renderer/react/index.tsx"),
    },
  },
  test: { environment: "node", include: ["index.ts"], includeSource: ["index.ts"], passWithNoTests: false },
});
