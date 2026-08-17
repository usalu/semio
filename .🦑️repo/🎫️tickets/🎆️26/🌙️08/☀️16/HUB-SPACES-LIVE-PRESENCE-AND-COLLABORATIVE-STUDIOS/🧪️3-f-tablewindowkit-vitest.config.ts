import { dirname, resolve } from "node:path";
import { defineConfig } from "vitest/config";

const repoRoot = "/Users/ueli/Documents/semio";
const targetFile = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/📊️table/🟦️component.ts");

export default defineConfig({
  root: repoRoot,
  resolve: {
    alias: {
      "@semio-tech/framework": resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts"),
    },
  },
  test: {
    name: "3-f-table-window-kit-probe",
    mode: "test",
    environment: "node",
    include: [targetFile],
    includeSource: [targetFile],
    passWithNoTests: false,
  },
});
