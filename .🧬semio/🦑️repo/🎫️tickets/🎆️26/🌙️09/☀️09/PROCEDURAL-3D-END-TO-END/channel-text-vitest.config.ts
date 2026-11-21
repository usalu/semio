import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

export default defineConfig({
  cacheDir: resolve(import.meta.dirname, "🗑️generated/channel-text-vitest"),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    include: [
      "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🪞️echo-pack/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx",
    ],
  },
});
