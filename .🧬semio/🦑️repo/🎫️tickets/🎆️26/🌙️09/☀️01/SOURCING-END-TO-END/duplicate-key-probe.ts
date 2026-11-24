import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔬️ Reproduces the duplicate-key warning on the first Curated add and prints the component stack. */
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"))("playwright");
const browser = await chromium.launch();
const page = await (await browser.newContext({ locale: "en-US", viewport: { width: 1800, height: 1100 } })).newPage();
await page.addInitScript(() => {
  const original = console.error;
  console.error = (...args: unknown[]) => {
    if (String(args[0]).includes("same key")) (window as any).__keys = [...((window as any).__keys ?? []), args.map(String).join(" | ") + "\n" + new Error().stack];
    original(...args);
  };
});
await page.goto("http://127.0.0.1:6081/");
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3000);
await page.getByRole("tabpanel", { name: "Pool", exact: true }).getByRole("row").filter({ hasText: "KVH C24 100×200" }).first().getByRole("button", { name: "Increase Curated", exact: true }).click();
await page.waitForTimeout(2500);
await page.getByRole("tabpanel", { name: "Curated", exact: true }).getByRole("row").filter({ hasText: "KVH C24 100×200" }).first().getByRole("button", { name: /increase/i }).click();
await page.waitForTimeout(2500);
console.log(JSON.stringify(await page.evaluate(() => (window as any).__keys), null, 1));
await browser.close();
