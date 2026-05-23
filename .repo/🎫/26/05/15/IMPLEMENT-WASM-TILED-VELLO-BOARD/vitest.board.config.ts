import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: [
      "elements/client/lib/board/index.ts",
      "elements/client/lib/board/index.tsx",
    ],
    passWithNoTests: false,
  },
});