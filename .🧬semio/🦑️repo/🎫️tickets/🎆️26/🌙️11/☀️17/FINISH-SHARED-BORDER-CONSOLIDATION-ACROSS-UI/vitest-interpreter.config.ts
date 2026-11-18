import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const os = resolve(repo, "🧰️framework/🛍️products/💻️os");
const uiLib = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local config running the UI Interpreter in-source suites (registered from its entry module under `import.meta.vitest`). */
export default defineConfig({
  root: os,
  server: { fs: { strict: false } },
  test: { include: [], includeSource: ["🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"], environment: "jsdom", setupFiles: [resolve(uiLib, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
