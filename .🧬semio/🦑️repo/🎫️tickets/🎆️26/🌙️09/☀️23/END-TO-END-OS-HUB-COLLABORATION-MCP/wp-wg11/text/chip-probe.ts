// 🔤️ WG11 T7 (ticket folder, temporary): Chromium's widths for the GraphTimeline checkpoint-hit chip labels (Anta 2xs, kerned) and
// how Chromium resolves `px-single` (3.2 px) — the sub-pixel inputs of the fixture's description boundary at x = 157.
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
const anta = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").toString("base64");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>@font-face{font-family:Anta;src:url(data:font/ttf;base64,${anta})} span{font-family:Anta;white-space:pre;font-size:9.6px}</style>`);
const out = await page.evaluate(async () => {
  await document.fonts.load('16px "Anta"');
  const w = (t: string) => { const s = document.createElement("span"); s.textContent = t; document.body.appendChild(s); const r = s.getBoundingClientRect().width; s.remove(); return r; };
  const pad = document.createElement("div"); pad.style.cssText = "display:inline-block;padding-left:3.2px;padding-right:3.2px"; document.body.appendChild(pad); const padW = pad.getBoundingClientRect().width; pad.remove();
  return { head: w("Head"), base: w("Base"), padding2x: padW };
});
await browser.close();
console.log(JSON.stringify(out));
