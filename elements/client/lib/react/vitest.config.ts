import { defineConfig } from "vitest/config";

// Use process.cwd() so this shared config works from any bundle root
// (elements/ui, semio/ui, …) without hardcoding the directory.
export default defineConfig({
  root: process.cwd(),
  test: {
    environment: "node",
    include: ["index.tsx"],
    includeSource: ["index.tsx"],
    passWithNoTests: true,
  },
});
