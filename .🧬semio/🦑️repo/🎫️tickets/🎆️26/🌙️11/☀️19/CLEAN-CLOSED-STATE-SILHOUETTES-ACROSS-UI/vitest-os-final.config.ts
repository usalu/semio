import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const ui = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local config for the os elements restyled in the final s/OS chrome round (task manager, agent chat and approvals, utility options, sync, staging dialogs). */
export default defineConfig({
  root,
  server: { fs: { strict: false } },
  test: { include: ["🧱️elements/🧵️TaskManager/🧪️tests/**/*.tsx", "🧱️elements/💬️AgentChatPanel/🧪️tests/**/*.tsx", "🧱️elements/🤖️AgentApprovals/🧪️tests/**/*.tsx", "🧱️elements/🔄️ShellSync/🧪️tests/**/*.tsx", "🧱️elements/🎛️UtilityTree/🧪️tests/**/*.tsx", "🧱️elements/🎓️HubFirstRun/🧪️tests/**/*.tsx"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
