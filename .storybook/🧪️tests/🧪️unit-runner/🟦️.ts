import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: [
      ".storybook/🧪️tests/🧪️catalog-integrity/🟦️.ts",
      ".storybook/🧪️tests/🧪️scope-resolution/🟦️.ts",
      ".storybook/🧪️tests/🧪️preparation/🟦️.ts",
    ],
    environment: "node",
    testTimeout: 60000,
  },
});
