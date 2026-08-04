#!/usr/bin/env bun
/** 📏 [DEBUG] temp: measure page-colored gap between consecutive full-width hairlines in a crop. */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const [pngPath] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const scale = 4; // crop.ts default

function lum(i: number) {
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

// Sample mid-band away from side borders / chip verticals: prefer the gap BETWEEN chips.
const x0 = Math.floor(img.width * 0.42);
const x1 = Math.floor(img.width * 0.58);
const rowRule: boolean[] = [];
const rowPage: boolean[] = [];
for (let y = 0; y < img.height; y++) {
  let rule = 0;
  let page = 0;
  for (let x = x0; x < x1; x++) {
    const i = (y * img.width + x) * 4;
    const L = lum(i);
    // hairline: mid grey (light theme ~120-180, dark theme ~140-200)
    if (L > 90 && L < 200) rule++;
    // page/cream or dark page strip (not canvas cell fill under headers)
    else if (L > 210 || L < 55) page++;
  }
  const span = x1 - x0;
  rowRule.push(rule > span * 0.45);
  rowPage.push(page > span * 0.55);
}

const rules: { y0: number; y1: number }[] = [];
for (let y = 0; y < img.height; y++) {
  if (!rowRule[y]) continue;
  const last = rules[rules.length - 1];
  if (last && y === last.y1 + 1) last.y1 = y;
  else rules.push({ y0: y, y1: y });
}

console.log(`[DEBUG] ${pngPath} ${img.width}x${img.height} midX=${x0}..${x1}`);
console.log(`[DEBUG] hairline bands: ${rules.map((r) => `${r.y0}-${r.y1}`).join(", ") || "(none)"}`);

for (let i = 0; i < rules.length - 1; i++) {
  const a = rules[i];
  const b = rules[i + 1];
  let pagePx = 0;
  for (let y = a.y1 + 1; y < b.y0; y++) if (rowPage[y]) pagePx++;
  const gapPx = b.y0 - a.y1 - 1;
  const gapPt = gapPx / scale;
  const pagePt = pagePx / scale;
  console.log(
    `[DEBUG] pair ${a.y0}-${a.y1} → ${b.y0}-${b.y1}: gap=${gapPt.toFixed(2)}pt page-strip=${pagePt.toFixed(2)}pt ${gapPx <= 0 ? "WELDED" : pagePx > 0 ? "GAP" : "adjacent"}`,
  );
}
