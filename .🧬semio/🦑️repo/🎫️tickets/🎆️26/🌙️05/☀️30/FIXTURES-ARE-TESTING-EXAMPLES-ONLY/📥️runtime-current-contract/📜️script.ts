import { test, expect } from "bun:test";
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const plugin = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests");
const caching = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching");
const output = join(dirname(import.meta.dir), "🗑️generated/runtime-current-contract");
mkdirSync(output, { recursive: true });
for (const [name, oracle] of [
  ["history-label-reload", "historyLabelReloadOracle"],
  ["time-travel", "timeTravelScenarioOracle"],
  ["supersede-ledger", "supersedeLedgerOracle"],
  ["composed-child-history", "composedChildHistoryOracle"],
  ["folder-reload-route", "folderReloadRouteOracle"],
  ["history-alternatives", "historyAlternativesOracle"],
]) test(`current domain twin ${name}`, async () => {
  const module = await import(join(plugin, `🧪️${name}/🟦️.ts`));
  const count = module[oracle!](root);
  expect(count).toBeGreaterThan(0);
  console.log(`[DEBUG] ${name} current semantic cases or steps=${count}`);
});
test("current Nx contract agrees with installed Nx", async () => {
  const { proveCachePolicy } = await import(join(caching, "🧪️tests/⚡️cache-contracts/🟦️.ts"));
  const { cacheInternals } = await import(join(caching, "../🟨️.mjs"));
  proveCachePolicy(root, cacheInternals);
  console.log("[DEBUG] current Nx policy domain comparison completed");
});
test("current native runtime publication and admission", async () => {
  const { testNativeRuntime } = await import(join(caching, "🧪️tests/🧊️native-runtime/🟦️.ts"));
  await testNativeRuntime(root, output);
  console.log("[DEBUG] current native runtime actual publication and admission completed");
}, 600000);
