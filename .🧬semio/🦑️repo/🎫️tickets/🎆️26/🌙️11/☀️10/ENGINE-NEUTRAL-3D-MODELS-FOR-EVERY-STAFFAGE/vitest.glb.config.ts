import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const glb = fileURLToPath(new URL("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/", import.meta.url));

const cache = fileURLToPath(new URL("🗑️generated/kernel-audit-fixes/vitest-cache/glb", import.meta.url));

export default defineConfig({
  cacheDir: cache,
  root: glb,
  test: { name: "glb-direct", environment: "node", include: ["🧪️tests/*/🟦️.ts"], exclude: ["**/🧰️support/**", "**/node_modules/**"] },
});
