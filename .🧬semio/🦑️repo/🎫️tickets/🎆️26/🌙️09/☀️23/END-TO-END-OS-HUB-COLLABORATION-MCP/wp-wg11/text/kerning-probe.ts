// 🔤️ WG11 T7b (ticket folder, temporary): Chromium's default (kerned) DOM width per corpus row vs an independent kerned sum
// (opentype.js hmtx + pair kerning) — proves the kerned oracle before the set commits it.
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
import opentype from "opentype.js";
const fonts = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts";
const files: Record<string, string> = { sans: `${fonts}/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, mono: `${fonts}/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf` };
const fixture = JSON.parse(readFileSync(process.argv[2], "utf8"));
const parsed = Object.fromEntries(Object.entries(files).map(([face, file]) => { const b = readFileSync(file); return [face, opentype.parse(b.buffer.slice(b.byteOffset, b.byteOffset + b.byteLength))]; }));
const css = Object.entries(files).map(([face, file]) => `@font-face{font-family:"${fixture.faces[face]}";src:url(data:font/ttf;base64,${readFileSync(file).toString("base64")})}`).join("");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>${css} span{white-space:pre}</style>`);
const widths: number[] = await page.evaluate(async ({ faces, rows }) => {
  for (const family of Object.values(faces) as string[]) await document.fonts.load(`16px "${family}"`);
  return rows.map((row: any) => { const s = document.createElement("span"); s.style.cssText = `font-family:"${faces[row.face]}";font-size:${row.sizePx}px`; s.textContent = row.text; document.body.appendChild(s); const w = s.getBoundingClientRect().width; s.remove(); return w; });
}, { faces: fixture.faces, rows: fixture.rows });
await browser.close();
let worst = 0; let kernedRows = 0;
fixture.rows.forEach((row: any, i: number) => {
  const font = parsed[row.face]; const glyphs = font.stringToGlyphs(row.text); const scale = row.sizePx / font.unitsPerEm;
  let units = 0; glyphs.forEach((g: any, j: number) => { units += g.advanceWidth; if (j + 1 < glyphs.length) units += font.getKerningValue(g, glyphs[j + 1]); });
  const oracle = units * scale; const delta = Math.abs(oracle - widths[i]); worst = Math.max(worst, delta);
  if (Math.abs(widths[i] - row.unkernedWidthPx) > 0.02) kernedRows++;
  if (delta > 0.05) console.log(`DELTA ${row.face} ${row.sizeToken} ${JSON.stringify(row.text)} chromium=${widths[i]} opentype=${oracle.toFixed(4)}`);
});
console.log(`rows=${fixture.rows.length} kernedRowsDifferingFromUnkerned=${kernedRows} worstOpentypeVsChromium=${worst.toFixed(4)}`);
if (process.argv[3]) { const { writeFileSync } = await import("node:fs"); writeFileSync(process.argv[3], JSON.stringify(widths)); }
