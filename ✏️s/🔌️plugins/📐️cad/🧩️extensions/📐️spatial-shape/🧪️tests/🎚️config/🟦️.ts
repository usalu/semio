// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const configDir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const root = resolve(configDir, "../.."); // extension root

export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/cad-js-module-spatial-shape",
    environment: "node",
    include: ["🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
  },
});
