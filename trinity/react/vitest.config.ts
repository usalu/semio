import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../..");

export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/ui-styling": resolve(repoRoot, "ui/styling/js/index.ts"),
      "@semio-tech/trinity-react": resolve(repoRoot, "trinity/react/index.tsx"),
    },
  },
  test: { environment: "node", include: ["index.tsx"], includeSource: ["index.tsx"], passWithNoTests: false },
});
