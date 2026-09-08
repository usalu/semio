// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const root = resolve(configDir, "../.."); // extension root

export default defineConfig({
  root,
  test: {
    name: "@semio-tech/cad-js-module-spatial-shape",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["🟦️.ts"] },
    includeSource: ["🟦️.ts"],
  },
});
