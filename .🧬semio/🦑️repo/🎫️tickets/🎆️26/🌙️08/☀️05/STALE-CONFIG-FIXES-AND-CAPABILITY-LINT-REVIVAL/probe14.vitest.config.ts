import { defineConfig } from "vitest/config";
export default defineConfig({
  test: {
    projects: [
      "./compose/client/lib/js/vite.config.ts",
      "./compose/dev/algorithm/js/vitest.config.ts",
      "./compose/client/lib/sketchpad/js/vitest.config.ts",
    ],
  },
});
