import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const model = fileURLToPath(new URL("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/", import.meta.url));

const cache = fileURLToPath(new URL("🗑️generated/kernel-audit-fixes/vitest-cache/model", import.meta.url));

export default defineConfig({
  cacheDir: cache,
  root: model,
  test: { name: "model-direct", environment: "node", include: ["🧪️tests/*/🟦️.ts"], exclude: ["**/🧰️support/**", "**/node_modules/**"], testTimeout: 120000 },
});
