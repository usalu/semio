import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const ui = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local config for canvas and plugin-composition suites outside the os and ui roots. */
export default defineConfig({
  root: repo,
  server: { fs: { strict: false } },
  test: { include: ["🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/🧪️tests/🪶️demand-frames/🟦️.tsx", "✏️s/🧪️tests/🏘️plugin-composition/🛍️layout-canvas-catalogue/🟦️.ts"], exclude: ["**/node_modules/**"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
