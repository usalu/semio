import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

export default defineConfig({
  cacheDir: resolve(import.meta.dirname, "🗑️generated/paid-inbound-vitest"),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    include: [
      "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️inbound-request/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🪝️extension-invocation/🟦️.ts",
    ],
  },
});
