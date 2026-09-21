import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    include: [resolve(import.meta.dirname, "../🟦️.ts")],
  },
});
