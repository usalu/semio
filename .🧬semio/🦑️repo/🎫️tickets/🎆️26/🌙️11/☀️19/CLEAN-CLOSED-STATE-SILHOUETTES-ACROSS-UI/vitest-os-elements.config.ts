import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const engine = resolve(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const ui = resolve(repo, "🧰️framework/🔨️modules/🖱️ui");
const names = ["🧩️BlockListHost", "🖌️Paint2dHost", "🛠️ShellHelpers", "🤖️AgentApprovals", "🏛️ShellHost", "🖋️InkCanvasHost", "🖥️Board2dHost", "🧭️TiledMapHost", "🕸️NodeGraph", "✏️TextEditor", "🎬️MediaTransportHost", "🗣️Interpreter"];

/** 🧪️ Ticket-local config for the OS element suites whose sources this sweep touched. */
export default defineConfig({
  root: engine,
  server: { fs: { strict: false } },
  test: { include: names.flatMap((name) => [`🧱️elements/${name}/**/*.test.ts`, `🧱️elements/${name}/**/*.test.tsx`, `🧱️elements/${name}/🧪️tests/**/*.ts`, `🧱️elements/${name}/🧪️tests/**/*.tsx`]), exclude: ["**/node_modules/**"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
