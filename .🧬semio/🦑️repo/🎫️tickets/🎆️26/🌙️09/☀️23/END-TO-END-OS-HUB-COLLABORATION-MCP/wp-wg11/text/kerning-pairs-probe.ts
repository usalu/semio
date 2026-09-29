// 🔤️ WG11 T7b (ticket folder, temporary): is Chromium's kerning PAIRWISE for the corpus? Each adjacent pair's kern is read in
// em at 1000 px (kerned − unkerned); a row's predicted kerned width = unkerned width + size × Σ pair kerns, vs Chromium's own.
import { chromium } from "playwright";
import { readFileSync, writeFileSync } from "node:fs";
const fonts = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts";
const files: Record<string, string> = { sans: `${fonts}/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, mono: `${fonts}/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf` };
const fixture = JSON.parse(readFileSync(process.argv[2], "utf8"));
const kerned: number[] = JSON.parse(readFileSync(process.argv[3], "utf8"));
const css = Object.entries(files).map(([face, file]) => `@font-face{font-family:"${fixture.faces[face]}";src:url(data:font/ttf;base64,${readFileSync(file).toString("base64")})}`).join("");
const pairs = new Map<string, { face: string; pair: string }>();
for (const row of fixture.rows) { const cs = [...row.text]; for (let i = 0; i + 1 < cs.length; i++) pairs.set(`${row.face}|${cs[i]}${cs[i + 1]}`, { face: row.face, pair: cs[i] + cs[i + 1] }); }
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.setContent(`<style>${css} span{white-space:pre;font-size:1000px}</style>`);
const kerns: Record<string, number> = await page.evaluate(async ({ faces, pairs }) => {
  for (const family of Object.values(faces) as string[]) await document.fonts.load(`16px "${family}"`);
  const width = (family: string, text: string, kerning: string) => { const s = document.createElement("span"); s.style.fontFamily = `"${family}"`; s.style.fontKerning = kerning; s.textContent = text; document.body.appendChild(s); const w = s.getBoundingClientRect().width; s.remove(); return w; };
  return Object.fromEntries(pairs.map(([key, { face, pair }]) => [key, (width(faces[face], pair, "normal") - width(faces[face], pair, "none")) / 1000]));
}, { faces: fixture.faces, pairs: [...pairs.entries()] });
await browser.close();
let worst = 0;
fixture.rows.forEach((row: any, i: number) => {
  const cs = [...row.text]; let em = 0; for (let j = 0; j + 1 < cs.length; j++) em += kerns[`${row.face}|${cs[j]}${cs[j + 1]}`];
  const predicted = row.unkernedWidthPx + row.sizePx * em; const delta = Math.abs(predicted - kerned[i]); worst = Math.max(worst, delta);
  if (delta > 0.05) console.log(`DELTA ${row.face} ${row.sizeToken} ${JSON.stringify(row.text)} chromium=${kerned[i]} pairwise=${predicted.toFixed(4)}`);
});
const nonzero = Object.entries(kerns).filter(([, v]) => Math.abs(v) > 1e-6);
console.log(`pairs=${pairs.size} kernedPairs=${nonzero.length} worstPairwiseVsChromium=${worst.toFixed(4)}`);
console.log(nonzero.slice(0, 60).map(([k, v]) => `${k}=${v.toFixed(4)}`).join(" "));
writeFileSync(process.argv[4], JSON.stringify(kerns, null, 1));
