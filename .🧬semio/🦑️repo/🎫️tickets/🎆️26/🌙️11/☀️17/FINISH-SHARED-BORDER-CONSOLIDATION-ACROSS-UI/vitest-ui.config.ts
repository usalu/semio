import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const ui = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local UI test config: jsdom with the repo's React test environment, bypassing the broken runner. */
export default defineConfig({
  root: ui,
  test: { include: ["**/🧪️tests/**/🟦️.{ts,tsx}", "**/*.test.{ts,tsx}"], exclude: ["**/node_modules/**", "**/🧪️owned-locale-detector-retirement/*/**"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
