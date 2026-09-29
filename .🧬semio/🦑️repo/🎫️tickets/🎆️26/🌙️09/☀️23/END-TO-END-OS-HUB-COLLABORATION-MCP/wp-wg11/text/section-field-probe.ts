// 🔤️ WG11 T7 (ticket folder, temporary): Chromium's own line count for the section/field fixture strings (Anta, kerned default) at the
// fixture's box widths — the fixture's `detailLines`/`titleLines` were authored against wgpu's old 0.625 em stand-in.
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
const anta = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").toString("base64");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>@font-face{font-family:Anta;src:url(data:font/ttf;base64,${anta})} div{font-family:Anta;overflow-wrap:break-word}</style>`);
const rows = await page.evaluate(async () => {
  await document.fonts.load('16px "Anta"');
  const probe = (text: string, width: number, size: number, lineHeight: number, weight: number) => {
    const div = document.createElement("div");
    div.style.cssText = `width:${width}px;font-size:${size}px;line-height:${lineHeight}px;font-weight:${weight}`;
    div.textContent = text;
    document.body.appendChild(div);
    const span = document.createElement("span");
    span.style.cssText = `font-family:Anta;font-size:${size}px;font-weight:${weight};white-space:pre`;
    span.textContent = text;
    document.body.appendChild(span);
    const out = { text, width, size, weight, lines: Math.round(div.getBoundingClientRect().height / lineHeight), singleLineWidth: span.getBoundingClientRect().width };
    div.remove(); span.remove();
    return out;
  };
  return [probe("Shown before control", 80, 11.2, 16, 400), probe("Fix this value now", 80, 11.2, 16, 400), probe("Shown before", 500, 11.2, 16, 400), probe("Fix this value", 500, 11.2, 16, 400), probe("Localized settings", 500, 21.6, 30.4, 400), probe("Threshold value", 112, 12.8, 19.2, 500), probe("Localized settings", 140, 21.6, 30.4, 600), probe("Localized settings", 500, 21.6, 30.4, 600)];
});
await browser.close();
for (const row of rows) console.log(JSON.stringify(row));
