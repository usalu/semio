import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: [
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/shared-border-dock.test.tsx",
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/**/🟦️.tsx",
      "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/**/🟦️.tsx",
    ],
    setupFiles: ["🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"],
    maxWorkers: 1,
    fileParallelism: false,
  },
  esbuild: { jsx: "automatic" },
});
