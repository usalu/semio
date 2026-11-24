import { mkdir } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🗄️ Scratch probe: attach a folder, curate, reload, reconnect (ticket archive persistence). */
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const url = "http://127.0.0.1:6081/";
const out = join(import.meta.dir, "🗑️generated", "archive");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({ locale: "en-US", viewport: { width: 1800, height: 1100 } });
const page = await context.newPage();
page.on("console", m => { if (/folder|backbone|archive|reconnect|mismatch|attach|identity/i.test(m.text()) && !/DEBUG/.test(m.text())) console.log("console:", m.text().slice(0, 240)); });
const folder = join(root, ".cache/b/p/m/sourcing-data", `probe-${Date.now()}`);
await page.goto(url);
console.log("beacon", await awaitBeacon(page, Date.now() + 300_000));
await dismissIntroduction(page); await page.waitForTimeout(3000);
await page.locator("#s-sync-status").click();
await page.locator("[id='ui.utilities.group.sync']").click();
await page.locator("[id='framework.sync.folder']").click();
await page.locator("[id='framework.sync.folder.path']").fill(folder);
await page.getByRole("button", { name: "Attach", exact: true }).click();
await page.waitForTimeout(3000);
const pool = page.getByRole("tabpanel", { name: "Pool", exact: true });
const curated = page.getByRole("tabpanel", { name: "Curated", exact: true });
await pool.getByRole("row").filter({ hasText: "Steel IPE 200" }).first().getByRole("button", { name: "Increase Curated", exact: true }).click().catch(async e => console.log("click", String(e).slice(0, 200)));
await page.waitForTimeout(4000);
const names = () => curated.locator("tbody tr").evaluateAll(rows => rows.map(r => r.textContent?.slice(0, 40)));
console.log("curated before", await names());
await page.screenshot({ path: join(out, "before.png") });
await page.reload();
console.log("beacon2", await awaitBeacon(page, Date.now() + 300_000));
await dismissIntroduction(page); await page.waitForTimeout(8000);
console.log("band", await page.evaluate(() => document.querySelector("[data-semio-folder-reconnect]")?.textContent ?? null));
await page.screenshot({ path: join(out, "after-reload.png") });
if (await page.locator("#s-folder-reconnect").count()) {
  await page.locator("#s-folder-reconnect").click();
  await page.waitForTimeout(8000);
  console.log("curated after reconnect", await names());
  await page.screenshot({ path: join(out, "after-reconnect.png") });
}
console.log("band after reconnect", await page.evaluate(() => document.querySelector("[data-semio-folder-reconnect]")?.getAttribute("data-semio-folder-reconnect") ?? null));
await pool.getByRole("row").filter({ hasText: "KVH C24" }).first().getByRole("button", { name: "Increase Curated", exact: true }).click();
await page.waitForTimeout(4000);
console.log("curated 2", await names());
await page.reload();
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page); await page.waitForTimeout(8000);
console.log("band2", await page.evaluate(() => document.querySelector("[data-semio-folder-reconnect]")?.textContent ?? null));
if (await page.locator("#s-folder-reconnect").count()) { await page.locator("#s-folder-reconnect").click(); await page.waitForTimeout(8000); }
console.log("curated 3", await names());
console.log("store", await page.evaluate(() => JSON.stringify(JSON.parse(JSON.parse(localStorage.getItem("semio.os.config")!).preferences["os.config.local-folders"]).events.map((e: any) => e.documentId))));
await browser.close();
