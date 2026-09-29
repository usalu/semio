// 🔤️ WG11 T7 finding (ticket folder, temporary): how much Chromium's SYNTHETIC bold (Anta ships one weight) widens a run, per
// glyph, at each size token and weight — wgpu's faux semibold keeps the regular advances.
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
const anta = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").toString("base64");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>@font-face{font-family:Anta;src:url(data:font/ttf;base64,${anta})} span{font-family:Anta;white-space:pre}</style>`);
const rows = await page.evaluate(async () => {
  await document.fonts.load('16px "Anta"');
  const text = "Localized settings";
  const width = (size: number, weight: number) => { const s = document.createElement("span"); s.style.cssText = `font-size:${size}px;font-weight:${weight}`; s.textContent = text; document.body.appendChild(s); const w = s.getBoundingClientRect().width; s.remove(); return w; };
  return [9.6, 11.2, 12.8, 14.4, 16, 21.6].map((size) => ({ size, perGlyph500: (width(size, 500) - width(size, 400)) / text.length, perGlyph600: (width(size, 600) - width(size, 400)) / text.length, perGlyph700: (width(size, 700) - width(size, 400)) / text.length }));
});
await browser.close();
for (const row of rows) console.log(JSON.stringify(row));
