import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: [
      "elements/client/lib/board/js/index.ts",
      "elements/client/lib/board/react/index.tsx",
    ],
    passWithNoTests: false,
  },
});