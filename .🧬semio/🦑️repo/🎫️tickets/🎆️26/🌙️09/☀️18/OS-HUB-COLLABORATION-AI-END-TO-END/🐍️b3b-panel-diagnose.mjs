/** 🔎️ Why does the framework History panel not open on one variant? Boots the given playground and
 * dumps every panel tab / panel button id plus the `#s-checkin` control, so the probe's
 * `framework.panel.history` selector can be checked against what the shell actually renders.
 * Usage: bun 🐍️b3b-panel-diagnose.mjs <variant> <port>
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";

const [variant, port] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const errors = [];
page.on("console", (m) => { if (m.type() === "error") errors.push(m.text().slice(0, 300)); });
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded", timeout: 180_000 });
let reloaded = false;
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (!reloaded && errors.some((e) => e.includes("Outdated Optimize Dep"))) { reloaded = true; await page.reload({ waitUntil: "domcontentloaded" }).catch(() => {}); continue; }
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")) && i > 8) break;
}
await page.waitForTimeout(4000);
const dump = async (label) => console.log(label, JSON.stringify(await page.evaluate(() => ({
  panelTabButtons: [...document.querySelectorAll('[data-slot="panel-tab-button"]')].map((el) => el.id),
  panelIds: [...document.querySelectorAll('[data-slot="panel"]')].map((el) => ({ id: el.id, visible: el.offsetParent !== null })),
  frameworkPanelIds: [...document.querySelectorAll('[id^="framework.panel"], [id^="framework.panelTab"]')].map((el) => `${el.tagName.toLowerCase()}#${el.id}`).slice(0, 40),
  anchors: [...document.querySelectorAll("[data-anchor]")].map((el) => `${el.getAttribute("data-anchor")}:${el.id}`).slice(0, 40),
  checkin: document.querySelector("#s-checkin")?.innerText ?? null,
  historyRows: document.querySelectorAll('[id^="framework.history.entry."]').length,
})), null, 1));
await dump("BOOT");
for (const selector of ['[data-slot="panel-tab-button"][id="framework.panel.history"]', '[id="framework.panel.history"]', '[id="framework.panelTab.framework.panel.history"]']) {
  const count = await page.locator(selector).count();
  if (count) { await page.locator(selector).first().click({ force: true, timeout: 8000 }).catch(() => {}); await page.waitForTimeout(1500); console.log("CLICKED", selector); }
  else console.log("ABSENT", selector);
}
await dump("AFTER");
console.log("ERRORS", JSON.stringify(errors.slice(0, 5)));
await browser.close();
