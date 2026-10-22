import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2] ?? "http://127.0.0.1:6029/", wait = Number(process.argv[3] ?? 120000);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const errors = [], logs = [];
page.on("console", m => { if (m.type() === "error") errors.push(m.text().slice(0, 300)); else logs.push(m.text().slice(0, 200)); });
page.on("pageerror", e => errors.push("pageerror " + String(e).slice(0, 300)));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
const t0 = Date.now();
let state = {};
while (Date.now() - t0 < wait) {
  state = await page.evaluate(() => ({ title: document.title, shells: [...document.querySelectorAll("[data-shell-id]")].map(e => [e.getAttribute("data-shell-id"), e.getAttribute("data-shell-ready"), e.getAttribute("data-shell-error")]), body: document.body.innerText.slice(0, 300) }));
  if (state.shells.length && state.shells.every(s => s[1] === "true" || s[2])) break;
  await page.waitForTimeout(2000);
}
const renderer = await page.evaluate(() => { const c = document.createElement("canvas"), g = c.getContext("webgl"), e = g && g.getExtension("WEBGL_debug_renderer_info"); return e ? g.getParameter(e.UNMASKED_RENDERER_WEBGL) : "none"; });
console.log(JSON.stringify({ url, seconds: (Date.now() - t0) / 1000, renderer, state, errors: errors.slice(0, 20), logs: logs.slice(-5) }, null, 1));
await page.screenshot({ path: process.argv[4] ?? "boot.png" });
await browser.close();
