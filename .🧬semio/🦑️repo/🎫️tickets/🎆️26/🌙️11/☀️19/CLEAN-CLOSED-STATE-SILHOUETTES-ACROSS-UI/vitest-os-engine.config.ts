import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const engine = resolve(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const ui = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local config for the OS engine test directory (ink, renderer forms, block list, paint) under jsdom. */
export default defineConfig({
  root: engine,
  server: { fs: { strict: false } },
  test: { include: ["🧪️tests/🖋️ink-canvas-domain-interaction/*.tsx", "🧪️tests/📋️renderer-types-and-forms/*.ts", "🧪️tests/*/🟦️.ts", "🧪️tests/*/🟦️.tsx"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
