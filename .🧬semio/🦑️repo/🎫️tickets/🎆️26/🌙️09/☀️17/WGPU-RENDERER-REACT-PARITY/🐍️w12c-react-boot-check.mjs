/** ⚛️🩺 React 6313 boot check: loads the host and reports its ready attribute plus every console error. */
import { chromium } from "playwright";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu","--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
await ctx.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS","1"); localStorage.setItem("semio.runtime.diagnostics","1"); } catch {} });
const page = await ctx.newPage();
const errors = [];
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") errors.push(`${m.type()} ${m.text()}`.slice(0, 400)); });
page.on("pageerror", (e) => errors.push(`pageerror ${String(e).slice(0, 400)}`));
await page.goto(process.env.SEMIO_REACT_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d", { waitUntil: "domcontentloaded", timeout: 180000 });
for (let s = 0; s < Number(process.env.SEMIO_REACT_BOOT ?? 420); s += 1) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), err: document.documentElement.getAttribute("data-semio-os-error"), controls: document.querySelectorAll("[id],button").length })).catch((e) => ({ ready: null, err: String(e).slice(0,200), controls: 0 }));
  if (s % 15 === 0 || state.ready) console.log(`${s}s ready=${state.ready} err=${state.err} controls=${state.controls}`);
  if (state.ready && state.controls > 4 && s > 4) { console.log(`BOOTED at ${s}s`); break; }
}
console.log("--- console errors ---");
for (const line of errors.slice(0, 25)) console.log(line);
await browser.close();
