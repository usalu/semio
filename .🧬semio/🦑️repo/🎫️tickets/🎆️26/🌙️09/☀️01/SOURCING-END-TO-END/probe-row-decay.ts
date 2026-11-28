import { join } from "node:path";
import { createRequire } from "node:module";
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await (await browser.newContext({ locale: "en-US", viewport: { width: 1800, height: 1100 } })).newPage();
const t0 = Date.now();
await page.goto("http://127.0.0.1:6081/");
await awaitBeacon(page, Date.now() + 300000); await dismissIntroduction(page);
await page.waitForTimeout(Number(process.env.WAIT ?? 3000));
const tog = (m) => page.getByRole("tabpanel",{name:"Pool",exact:true}).locator(`[id="window:sourcing-pool/sourcing-filter-module-${m}"]`);
const state = async (tag) => console.log(tag, await page.evaluate(() => ["beams","windows","slabs","reuse"].map(m => m[0] + (document.querySelector(`[role=tabpanel][aria-label="Pool"] [id="window:sourcing-pool/sourcing-filter-module-${m}"]`)?.getAttribute("aria-pressed") === "true" ? "+" : "-")).join(" ") + " rows=" + document.querySelectorAll("[role=tabpanel] tbody tr").length));
await state("start");
for (const m of ["beams", "windows"]) { await tog(m).click(); await state("on " + m); await page.waitForTimeout(Number(process.env.GAP ?? 0)); }
await page.waitForTimeout(1500); await state("both on");
for (const m of ["beams", "windows"]) { await tog(m).click(); await state("off " + m); await page.waitForTimeout(Number(process.env.GAP ?? 0)); }
await page.waitForTimeout(3000); await state("end");
await browser.close();
