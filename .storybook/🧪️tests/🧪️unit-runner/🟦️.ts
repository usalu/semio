import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    include: [
      ".storybook/🧪️tests/🧪️workspace-exports/🟦️.ts",
      ".storybook/🧪️tests/🧪️mdx-discovery/🟦️.ts",
      ".storybook/🧪️tests/🧪️browser-discovery/🟦️.ts",
      ".storybook/🧪️tests/🧪️catalog-integrity/🟦️.ts",
      ".storybook/🧪️tests/🧪️scope-resolution/🟦️.ts",
      ".storybook/🧪️tests/🧪️preparation/🟦️.ts",
    ],
    environment: "node",
    testTimeout: 60000,
  },
});
