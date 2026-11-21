import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  cacheDir: resolve(import.meta.dirname, "🗑️generated/direct-hover-vitest"),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    include: [],
    maxWorkers: 1,
    includeSource: ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"],
  },
});
