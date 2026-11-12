import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const fbx = fileURLToPath(new URL("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/", import.meta.url));

const cache = fileURLToPath(new URL("🗑️generated/kernel-audit-fixes/vitest-cache/fbx", import.meta.url));

export default defineConfig({
  cacheDir: cache,
  root: fbx,
  test: { name: "fbx-direct", environment: "node", include: ["🧪️tests/*/🟦️.ts"], exclude: ["🧪️tests/🧰️support/🟦️.ts"], passWithNoTests: false, testTimeout: 120000 },
});
