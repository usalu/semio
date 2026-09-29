// 🔤️ WG11 T7a/T7b (ticket folder, temporary): writes the neutral text-advance corpus with Chromium's UNKERNED DOM widths (T7a) and,
// with `--with-kerned`, Chromium's default KERNED widths too (T7b).
import { chromium } from "playwright";
import { readFileSync, writeFileSync } from "node:fs";
const fonts = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts";
const faces = { sans: { family: "Anta", file: `${fonts}/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf` }, mono: { family: "Share Tech Mono", file: `${fonts}/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf` } };
const sizes = { "2xs": 9.6, xs: 11.2, sm: 12.8, base: 14.4, lg: 16, "2xl": 21.6 };
const strings = [
  { locale: "en", text: "Settings" }, { locale: "de", text: "Einstellungen" },
  { locale: "en", text: "Marketplace · 1.2.0 · loaded" }, { locale: "de", text: "Marktplatz · 1.2.0 · geladen" },
  { locale: "en", text: "feature-editor" }, { locale: "de", text: "Grüße aus Zürich" },
  { locale: "en", text: "The quick brown fox jumps over the lazy dog" }, { locale: "de", text: "Größenänderung übernehmen" },
  { locale: "en", text: "0123456789 +-*/%" }, { locale: "de", text: "Straße, Maß & Fuß" },
];
const out = process.argv[2];
const withKerned = process.argv.includes("--with-kerned");
if (withKerned) strings.push({ locale: "en", text: "AVATAR To Wave" }, { locale: "de", text: "VATER, Tätowierung" });
const css = Object.values(faces).map(({ family, file }) => `@font-face{font-family:"${family}";src:url(data:font/ttf;base64,${readFileSync(file).toString("base64")})}`).join("");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>${css} span{white-space:pre}</style>`);
const rows = await page.evaluate(async ({ faces, sizes, strings, withKerned }) => {
  for (const { family } of Object.values(faces)) await document.fonts.load(`16px "${family}"`);
  const rows = [];
  for (const [face, { family }] of Object.entries(faces)) for (const [token, size] of Object.entries(sizes)) for (const { locale, text } of strings) {
    const width = (kerning: string) => {
      const span = document.createElement("span");
      span.style.cssText = `font-family:"${family}";font-size:${size}px;font-kerning:${kerning}`;
      span.textContent = text;
      document.body.appendChild(span);
      const measured = span.getBoundingClientRect().width;
      span.remove();
      return measured;
    };
    rows.push({ face, sizeToken: token, sizePx: size, locale, text, unkernedWidthPx: width("none"), ...(withKerned ? { kernedWidthPx: width("normal") } : {}) });
  }
  return rows;
}, { faces, sizes, strings, withKerned });
await browser.close();
const fixture = { schemaVersion: 1, oracle: withKerned ? "Chromium DOM width: unkernedWidthPx with font-kerning none, kernedWidthPx with the default kerning" : "Chromium DOM width, font-kerning: none", tolerancePx: 0.5, faces: Object.fromEntries(Object.entries(faces).map(([face, { family }]) => [face, family])), rows };
writeFileSync(out, JSON.stringify(fixture, null, 2) + "\n");
console.log(`rows=${rows.length}`);
