import { defineConfig } from "vitest/config";
export default defineConfig({
  test: {
    name: "scopes-test",
    environment: "node",
    include: [".storybook/scopes.ts"],
    includeSource: [".storybook/scopes.ts"],
  },
});
