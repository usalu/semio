import { defineConfig } from "vitest/config";

// Specs: Same as {@link ../../elements/ui/vitest.config.ts}; local file so `root` resolves to semio/ui when npm test runs here.
// Summary: Vitest config for @semio/ui.

export default defineConfig({
  root: process.cwd(),
  test: {
    environment: "node",
    include: ["index.tsx"],
    passWithNoTests: true,
  },
});
