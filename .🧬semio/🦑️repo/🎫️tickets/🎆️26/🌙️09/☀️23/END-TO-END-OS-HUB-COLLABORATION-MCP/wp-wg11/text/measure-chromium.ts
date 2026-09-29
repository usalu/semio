// 🔤️ WG11 T7 feasibility probe: Chromium's DOM text widths for a small corpus, kerned (the React default) and unkerned.
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
const fonts = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts";
const faces = { Anta: `${fonts}/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, Mono: `${fonts}/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf` };
const strings = ["feature-editor", "feature-hosts", "checkpoint", "Werkstatt Ada", "Einstellungen", "AVATAR To Wave", "Grüße, Welt"];
const sizes = [9.6, 11.2, 12.8, 14.4, 16, 21.6];
const css = Object.entries(faces).map(([name, path]) => `@font-face{font-family:${name};src:url(data:font/ttf;base64,${readFileSync(path).toString("base64")})}`).join("");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>${css} span{white-space:pre}</style><div id=root></div>`);
const out = await page.evaluate(async ({ strings, sizes }) => {
  await document.fonts.load("16px Anta"); await document.fonts.load("16px Mono");
  const rows = [];
  for (const family of ["Anta", "Mono"]) for (const size of sizes) for (const text of strings) {
    const measure = (kerning: string) => { const s = document.createElement("span"); s.style.cssText = `font-family:${family};font-size:${size}px;font-kerning:${kerning}`; s.textContent = text; document.body.appendChild(s); const w = s.getBoundingClientRect().width; s.remove(); return w; };
    rows.push({ family, size, text, kerned: measure("auto"), unkerned: measure("none") });
  }
  return rows;
}, { strings, sizes });
await browser.close();
console.log(JSON.stringify(out));
