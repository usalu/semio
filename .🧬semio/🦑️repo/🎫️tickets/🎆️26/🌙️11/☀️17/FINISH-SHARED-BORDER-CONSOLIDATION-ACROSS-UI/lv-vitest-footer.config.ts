import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const os = resolve(repo, "🧰️framework/🛍️products/💻️os");
const uiLib = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local os-product test config: jsdom with the repo's React test environment, bypassing the broken runner. */
export default defineConfig({
  root: os,
  server: { fs: { strict: false } },
  test: { include: ["**/🏛️ShellFooter/🟦️.test.tsx"], exclude: ["**/node_modules/**"], environment: "jsdom", setupFiles: [resolve(uiLib, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
