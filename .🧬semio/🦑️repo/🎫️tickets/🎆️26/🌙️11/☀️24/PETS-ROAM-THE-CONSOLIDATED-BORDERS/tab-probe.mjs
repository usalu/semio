/** ⌨️ Replays on the dev page the keyboard walk of the deeds spec: the long name in the first row of deeds, the focus on its first button, then Tab after Tab, saying where the focus is each time. Usage: node tab-probe.mjs [width] [name] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[2] ?? 320), height: 2400 } });
const name = process.argv[3] ?? "";
await page.goto("http://127.0.0.1:6061/#prefs");
await page.waitForSelector("[data-pets-player]", { timeout: 180000 });
await page.evaluate(() => { const root = document.documentElement; root.style.fontSize = `${Number.parseFloat(getComputedStyle(root).fontSize) * 2}px`; });
await page.waitForTimeout(6000);
const where = () => page.evaluate(() => { const a = document.activeElement; const row = a?.closest("[data-pets-player]"); const r = a?.getBoundingClientRect(); return (a?.tagName ?? "none") + " " + JSON.stringify((a?.textContent ?? "").trim().slice(0, 30)) + " row=" + (row?.getAttribute("data-pets-player") ?? "-") + " index=" + (row ? [...row.querySelectorAll("button")].indexOf(a) : -1) + " box=" + (r ? [r.x, r.y, r.width, r.height].map(Math.round).join(",") : "") + " rows=" + document.querySelectorAll("[data-pets-player]").length + " scrollY=" + Math.round(document.querySelector(".course-app").scrollTop); });
if (name !== "") await page.evaluate((name) => { document.querySelector("[data-pets-player] span[id]").textContent = name; }, name);
await page.locator("[data-pets-player]").first().getByRole("button").first().focus();
console.log("focus: " + (await where()));
for (let step = 1; step <= 8; step += 1) { await page.keyboard.press("Tab"); await page.waitForTimeout(400); console.log("tab " + step + ": " + (await where())); }
await browser.close();
