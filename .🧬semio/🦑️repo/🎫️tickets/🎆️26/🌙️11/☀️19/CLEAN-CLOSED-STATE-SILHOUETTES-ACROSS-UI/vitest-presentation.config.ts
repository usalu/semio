import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/🎤️presentation");

/** 🧪️ Ticket-local presentation test config: jsdom with the presentation React setup, bypassing the repo runner. */
export default defineConfig({
  root,
  server: { fs: { strict: false } },
  test: { root, environment: "jsdom", include: ["🧪️tests/**/🟦️.{ts,tsx}", "🎯️targets/⚛️react/🧪️tests/**/🟦️.{ts,tsx}"], exclude: ["**/node_modules/**", "**/🧱️*-kit/**", "**/🧫️fixtures/**", "**/🧰️vitest-setup/**", "**/🎚️config/**"], setupFiles: [resolve(root, "🎯️targets/⚛️react/🧪️tests/🧰️vitest-setup/🟦️.ts")], testTimeout: 60000 },
});
