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
page.on("console", m => { const t = m.text(); if (!m.type || true) console.log("C", Date.now() - t0, t.slice(0, 2500)); });
page.on("pageerror", e => console.log("PAGEERR", String(e).slice(0, 300)));
page.on("worker", w => { (w as any).on?.("console", (m: any) => { const t = m.text(); if (!m.type || true) console.log("W", Date.now() - t0, t.slice(0, 3500)); }); });


await page.goto("http://127.0.0.1:6081/");
await awaitBeacon(page, Date.now() + 300000); await dismissIntroduction(page);
await page.waitForTimeout(3000);
const tog = (m) => page.getByRole("tabpanel",{name:"Pool",exact:true}).locator(`[id="window:sourcing-pool/sourcing-filter-module-${m}"]`);
for (const m of ["beams", "windows"]) await tog(m).click();
await page.waitForTimeout(Number(process.env.WAIT ?? 6000));
console.log("ROWS", await page.getByRole("tabpanel",{name:"Pool",exact:true}).getByRole("row").count());
console.log("PRESSED", await tog("beams").getAttribute("aria-pressed"), await tog("windows").getAttribute("aria-pressed"));
console.log("ledger", JSON.stringify(await page.evaluate(() => (globalThis.__semioInputLedger?.recent ?? []).slice(-4).map(r => ({ s: r.inputSeq, a: r.action, o: r.outcome?.kind ?? null })))));
for (const w of page.workers()) console.log("ILS", JSON.stringify(await w.evaluate(() => (globalThis as any).__ils ?? null).catch(() => "err")));
console.log("workers", page.workers().map(w => w.url().slice(-80)));
await browser.close();
