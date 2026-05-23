import { defineConfig } from "vitest/config";

// Specs: Same pattern as {@link ../../../../../elements/client/lib/react/vitest.config.ts}; local file so `root` resolves to this package when Nx runs tests here.
// Summary: Vitest config for @semio/ui.

export default defineConfig({
  root: process.cwd(),
  test: {
    environment: "node",
    include: ["index.tsx"],
    passWithNoTests: true,
  },
});
