import { defineConfig } from "vitest/config";
import { fileURLToPath } from "node:url";

export default defineConfig({
  cacheDir: fileURLToPath(new URL("./🗑️generated/vitest", import.meta.url)),
  test: {
    environment: "jsdom",
    include: [
      "🧰️framework/🔨️modules/🖱️ui/**/shared-border*.test.{ts,tsx}",
      "🧰️framework/🔨️modules/🖱️ui/**/border*.test.{ts,tsx}",
      "🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/🧪️tests/**/🟦️.ts",
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/{🎛️ToggleGroup,🔀️Toggle,🗨️Popover,🖱️ContextMenu,🗂️WindowChrome,🔲️WindowSilhouette}/🧪️tests/🧩️component/{🟦️.tsx,placement.tsx}",
      "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx",
    ],
    setupFiles: ["🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"],
    maxWorkers: 1,
    fileParallelism: false,
    testTimeout: 15000,
  },
  esbuild: { jsx: "automatic" },
});
