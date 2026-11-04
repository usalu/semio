import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: ["🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧩️component/🟦️.tsx"],
    setupFiles: ["🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"],
    maxWorkers: 1,
    fileParallelism: false,
  },
  esbuild: { jsx: "automatic" },
});
