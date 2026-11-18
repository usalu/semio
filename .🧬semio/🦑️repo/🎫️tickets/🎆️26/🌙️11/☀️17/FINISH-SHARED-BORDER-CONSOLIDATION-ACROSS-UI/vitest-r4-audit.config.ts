import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const ui = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local config for the round 4 audit fixes outside the course and the shared-border-row suites. */
export default defineConfig({
  root: repo,
  server: { fs: { strict: false } },
  test: {
    include: ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/dialog-borders.test.ts", "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/**/*.test.{ts,tsx}", "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow/*.test.{ts,tsx}"],
    environment: "jsdom",
    setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")],
    testTimeout: 30000,
  },
});
