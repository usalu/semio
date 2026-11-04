import { defineConfig } from "vitest/config";
import { fileURLToPath } from "node:url";
export default defineConfig({
  cacheDir: fileURLToPath(new URL("./🗑️generated/select-vite", import.meta.url)),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    globals: true,
    include: ["🧰️framework/🔨️modules/🖱️ui/🧱️elements/{🔽️Select,📑️Tabs}/🧪️tests/🧩️component/{🟦️,boundary}.tsx"],
    setupFiles: ["🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"],
  },
});
