import { defineConfig } from "vitest/config";
export default defineConfig({
  test: {
    projects: [
      "./compose/client/lib/js/vite.config.ts",
    ],
  },
});
