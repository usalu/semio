/** 🔎️ C8 — does the gis2d inspector re-project after a LOCAL addFeature? Usage: bun c8-inspector-local.mjs <shellUrl> <tag> */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";
const SHELL = process.argv[2]; const TAG = process.argv[3] ?? "c8i";
const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c8/generated";
const t0 = Date.now(); const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 500)}`));
const click = async (sel) => (await page.locator(sel).count()) ? page.locator(sel).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 80)) : "absent";
const rows = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => el.offsetParent !== null && !el.id.includes("framework.panel.history")).flatMap((p) => [...p.querySelectorAll('[role="treeitem"]')].map((el) => (el.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 60))));
await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let i = 0; i < 200; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
for (const t of await page.locator('[id$=".engagement.toggle"]').all()) { await t.click({ timeout: 6000, force: true }).catch(() => {}); await page.waitForTimeout(700); }
console.log("tab", await click('[data-slot="panel-tab-button"][id="framework.panel.inspection"], button:has-text("Inspection")'));
await page.waitForTimeout(3000);
console.log("before", JSON.stringify((await rows()).slice(0, 4)));
console.log("open", await click('[id="action.addFeature"]')); await page.waitForTimeout(1200);
console.log("exec", await click('[id$=".action.addFeature.execute"]'));
for (const wait of [2000, 5000, 10000]) { await page.waitForTimeout(wait); console.log(`after+${wait}`, JSON.stringify((await rows()).slice(0, 4))); }
await page.screenshot({ path: `${OUT}/${TAG}.png` });
writeFileSync(`${OUT}/${TAG}-console.txt`, lines.join("\n"));
await browser.close();
