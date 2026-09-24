/** 🔎️ C10 boot probe: loads one shell URL, records failed requests, console errors and the shell's ready/error attributes, screenshots. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";
const url = process.argv[2], tag = process.argv[3] ?? "boot", waitMs = Number(process.argv[4] ?? 60000);
const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c10/generated";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const t0 = Date.now();
page.on("console", (m) => lines.push(`${Date.now() - t0} console.${m.type()} ${m.text().slice(0, 500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 800)}`));
page.on("response", (r) => { if (r.status() >= 400) lines.push(`${Date.now() - t0} http ${r.status()} ${r.url().slice(0, 300)}`); });
page.on("requestfailed", (r) => lines.push(`${Date.now() - t0} failed ${r.url().slice(0, 300)} ${r.failure()?.errorText}`));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
const deadline = Date.now() + waitMs;
let state = {};
while (Date.now() < deadline) {
  state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), error: document.documentElement.getAttribute("data-semio-os-error"), tables: document.querySelectorAll(".semio-table-host").length, signIn: document.querySelectorAll('[data-semio-hub-sign-in]').length, text: document.body.innerText.slice(0, 400) }));
  if (state.tables > 0 || state.error) break;
  await page.waitForTimeout(1000);
}
await page.screenshot({ path: `${OUT}/${tag}.png` });
writeFileSync(`${OUT}/${tag}.txt`, `${JSON.stringify(state, null, 1)}\n${lines.join("\n")}\n`);
console.log(JSON.stringify(state));
await browser.close();
