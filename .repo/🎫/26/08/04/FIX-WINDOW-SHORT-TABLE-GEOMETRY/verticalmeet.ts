#!/usr/bin/env bun
/** 📐 [DEBUG] temp: check chip side verticals meet the shared baseline (no floating). */
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
function isRule(L: number) {
  return L > 95 && L < 185;
}

// Find full-width baseline candidates in the upper half.
const midX0 = Math.floor(img.width * 0.4);
const midX1 = Math.floor(img.width * 0.6);
const rules: number[] = [];
for (let y = 0; y < Math.floor(img.height * 0.7); y++) {
  let n = 0;
  for (let x = midX0; x < midX1; x++) if (isRule(lum(x, y))) n++;
  if (n > (midX1 - midX0) * 0.45) rules.push(y);
}
const bands: { y0: number; y1: number }[] = [];
for (const y of rules) {
  const last = bands[bands.length - 1];
  if (last && y === last.y1 + 1) last.y1 = y;
  else bands.push({ y0: y, y1: y });
}

// Chip baseline = first mid-band rule that sits BELOW chip text ink.
let inkBottom = -1;
const inkX0 = Math.floor(img.width * 0.08);
const inkX1 = Math.floor(img.width * 0.35);
for (let y = 0; y < img.height; y++) {
  let ink = 0;
  for (let x = inkX0; x < inkX1; x++) {
    const L = lum(x, y);
    if (L < 80 || L > 200) {
      /* dark text or light text on dark theme */
    }
    if (L < 80) ink++;
    if (L > 200 && L < 250) {
      /* skip */
    }
    // dark-theme text is light
    if (L > 180 && L < 250) {
      const bg = lum(Math.floor(img.width * 0.5), y);
      if (bg < 80) ink++; // light glyphs on dark page
    }
  }
  if (ink > 8) inkBottom = y;
  if (inkBottom >= 0 && y > inkBottom + Math.ceil(20 * scale)) break;
}
const baseline = bands.find((b) => b.y0 > inkBottom) ?? bands[bands.length - 1];
if (!baseline || inkBottom < 0) {
  console.log(`[DEBUG] ${pngPath}: no baseline found (inkBottom=${inkBottom})`);
  process.exit(0);
}
console.log(`[DEBUG] inkBottom y=${inkBottom}`);

// Find left chip's right vertical (first vertical rule cluster from left in chip band).
const chipTop = Math.max(0, baseline.y0 - Math.floor(18 * scale));
const chipBot = baseline.y0 - 1;
const vertXs: number[] = [];
for (let x = 0; x < Math.floor(img.width * 0.5); x++) {
  let n = 0;
  for (let y = chipTop; y <= chipBot; y++) if (isRule(lum(x, y))) n++;
  if (n > (chipBot - chipTop + 1) * 0.35) vertXs.push(x);
}
const vertBands: { x0: number; x1: number }[] = [];
for (const x of vertXs) {
  const last = vertBands[vertBands.length - 1];
  if (last && x === last.x1 + 1) last.x1 = x;
  else vertBands.push({ x0: x, x1: x });
}

console.log(`[DEBUG] ${pngPath}`);
console.log(`[DEBUG] baseline y=${baseline.y0}-${baseline.y1} (${((baseline.y1 - baseline.y0 + 1) / scale).toFixed(2)}pt)`);
console.log(`[DEBUG] verticals: ${vertBands.map((v) => `${v.x0}-${v.x1}`).join(", ") || "(none)"}`);

for (const v of vertBands.slice(0, 4)) {
  const x = Math.floor((v.x0 + v.x1) / 2);
  // Walk down from chip bot toward baseline: count page pixels before rule.
  let gap = 0;
  let y = chipBot;
  while (y < baseline.y0) {
    const L = lum(x, y);
    if (isRule(L)) break;
    gap++;
    y++;
  }
  // Also check if vertical ink exists on the baseline row (T-junction).
  let onBaseline = false;
  for (let yy = baseline.y0; yy <= baseline.y1; yy++) {
    if (isRule(lum(x, yy))) onBaseline = true;
  }
  console.log(
    `[DEBUG] vert x=${x}: gap-to-baseline=${(gap / scale).toFixed(2)}pt onBaseline=${onBaseline} ${gap <= 1 ? "WELDED" : "FLOATING"}`,
  );
}

// Between chips: any second parallel rule within 3pt below baseline?
const after = bands.find((b) => b.y0 > baseline.y1 && b.y0 <= baseline.y1 + Math.ceil(3 * scale));
if (after) {
  const gap = after.y0 - baseline.y1 - 1;
  console.log(`[DEBUG] SECOND hairline at y=${after.y0}-${after.y1} gap=${(gap / scale).toFixed(2)}pt DOUBLE`);
} else {
  console.log(`[DEBUG] no second hairline within 3pt — single shared line`);
}
