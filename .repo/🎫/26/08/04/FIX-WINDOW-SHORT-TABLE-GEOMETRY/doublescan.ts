#!/usr/bin/env bun
/** 🔬 [DEBUG] temp: under-chip scan for two hairlines separated by page pixels. */
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
const scale = 4;

function lum(x: number, y: number) {
  const i = (y * img.width + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

// Sample several x columns under the left chip (avoid vertical borders).
const xs = [0.12, 0.16, 0.20, 0.24].map((f) => Math.floor(img.width * f));

type RowKind = "rule" | "page" | "other";
function classifyRow(y: number): RowKind {
  let rule = 0, page = 0;
  for (const x of xs) {
    const L = lum(x, y);
    if (L > 100 && L < 170) rule++;
    else if (L > 210 || L < 40) page++;
  }
  if (rule >= 3) return "rule";
  if (page >= 3) return "page";
  return "other";
}

const kinds = Array.from({ length: img.height }, (_, y) => classifyRow(y));
const bands: { kind: RowKind; y0: number; y1: number }[] = [];
for (let y = 0; y < kinds.length; y++) {
  const last = bands[bands.length - 1];
  if (last && last.kind === kinds[y] && y === last.y1 + 1) last.y1 = y;
  else bands.push({ kind: kinds[y], y0: y, y1: y });
}

console.log(`[DEBUG] ${pngPath}`);
const ruleBands = bands.filter((b) => b.kind === "rule");
console.log(`[DEBUG] rule bands: ${ruleBands.map((b) => `${b.y0}-${b.y1}(${((b.y1 - b.y0 + 1) / scale).toFixed(2)}pt)`).join(", ")}`);

let found = false;
for (let i = 0; i < ruleBands.length - 1; i++) {
  const a = ruleBands[i];
  const b = ruleBands[i + 1];
  const gapPx = b.y0 - a.y1 - 1;
  if (gapPx > 12 * scale) continue; // only nearby pairs
  let pagePx = 0;
  for (let y = a.y1 + 1; y < b.y0; y++) if (kinds[y] === "page") pagePx++;
  const gapPt = gapPx / scale;
  const pagePt = pagePx / scale;
  const verdict = gapPx <= 0 ? "WELDED" : pagePx > 0 ? "DOUBLE+GAP" : "ADJACENT";
  console.log(`[DEBUG] pair ${a.y0}-${a.y1} → ${b.y0}-${b.y1}: gap=${gapPt.toFixed(2)}pt page=${pagePt.toFixed(2)}pt ${verdict}`);
  if (gapPx > 0 && gapPx <= 3 * scale) found = true;
}
if (!found) console.log(`[DEBUG] RESULT: no nearby double hairline under chip (OK)`);
else console.log(`[DEBUG] RESULT: FAIL — double hairline with gap under chip`);

// Also compare to known-bad
console.log(`[DEBUG] solid-rule thickness of first band under mid-height: check top half`);
const topRules = ruleBands.filter((b) => b.y1 < img.height * 0.55);
if (topRules[0]) {
  const t = (topRules[0].y1 - topRules[0].y0 + 1) / scale;
  console.log(`[DEBUG] first top rule thickness=${t.toFixed(2)}pt (expect ~0.75 single, ~1.5 double)`);
}
