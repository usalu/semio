/** 🔎️ Dumps the Actions-pane form DOM (ids, tags, roles) of one action row so interaction probes can target its controls.
 * Usage: cd <ticket> && SEMIO_PROBE_ACTION=patchNodes bun 🐍️trinity-form-dom-probe.mjs
 */
import { chromium } from "playwright";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6054/?plugin=trinity-jack";
const action = process.env.SEMIO_PROBE_ACTION ?? "patchNodes";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 200; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
await page.waitForTimeout(3000);
const engagement = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id)[0]);
await page.locator(`[id="${engagement}.toggle"]`).first().click({ force: true });
await page.waitForTimeout(1500);
await page.locator(`[id="action.${action}"]`).first().click({ force: true });
await page.waitForTimeout(2000);
const dom = await page.evaluate((action) => [...document.querySelectorAll(`[id*="${action}"]`)].map((el) => ({ id: el.id, tag: el.tagName, role: el.getAttribute("role"), type: el.getAttribute("type"), editable: el.getAttribute("contenteditable"), inner: [...el.querySelectorAll("input,textarea,select,[contenteditable],[role=combobox],button")].map((c) => `${c.tagName}#${c.id}[${c.getAttribute("role") ?? ""}]`).slice(0, 6) })), action);
console.log("[DEBUG] FORM", JSON.stringify(dom, null, 1));
await browser.close();
