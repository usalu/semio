import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const ui = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🔨️modules/🖱️ui");

/** 🧪️ Ticket-local os elements test config: jsdom with the repo's React test environment, bypassing the broken runner. */
export default defineConfig({
  root,
  server: { fs: { strict: false } },
  test: { include: ["🧪️tests/🧩️block-list-presentation/🟦️.tsx", "🧪️tests/🏛️space-administration/🟦️.tsx", "🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx", "🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🖱️selection-focus/🟦️.tsx"], environment: "jsdom", setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")], testTimeout: 30000 },
});
