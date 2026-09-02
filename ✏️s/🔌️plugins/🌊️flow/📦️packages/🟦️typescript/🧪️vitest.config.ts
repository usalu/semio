import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/flow-js",
    environment: "node",
    include: ["🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    passWithNoTests: true,
  },
});
