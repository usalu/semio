import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/flow-js",
    environment: "node",
    include: ["📦️index.ts"],
    coverage: { include: ["📦️index.ts"] },
    passWithNoTests: true,
  },
});
