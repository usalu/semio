import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const cache = fileURLToPath(new URL("🗑️generated/kernel-audit-fixes/vitest-cache/viewer", import.meta.url));

export default defineConfig({
  cacheDir: cache,
  root: "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer",
  test: { name: "viewer-direct", environment: "node", include: ["🧪️tests/*/🟦️.ts"], exclude: ["**/🧰️support/**", "**/node_modules/**"] },
});
