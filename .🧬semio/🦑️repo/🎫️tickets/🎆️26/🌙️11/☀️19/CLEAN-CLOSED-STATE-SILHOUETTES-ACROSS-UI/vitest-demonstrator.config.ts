import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../♻️mit-bestand/🧺️demonstrator");

/** 🧪️ Ticket-local demonstrator unit config: node environment, shell-brand and in-source brand/pane tests, bypassing the repo runner. */
export default defineConfig({
  root,
  server: { fs: { strict: false } },
  test: {
    root,
    environment: "node",
    maxWorkers: 1,
    include: ["./🧪️tests/🏛️shell-brand/🟦️.ts"],
    includeSource: ["./🪧️brand.ts", "./🧪️tests/🧪️demonstratorpanebranding/🟦️.ts"],
    testTimeout: 60000,
  },
});
