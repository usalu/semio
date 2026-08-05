#!/usr/bin/env bun
/** 🔬 [DEBUG] Ground-truth pixel metrics: chip seam, photo pad, L-border joins. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, outPrefix] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;
const page = await doc.getPage(Number(pageArg));
const scale = 8;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await page.render({ canvasContext: ctx, viewport }).promise;
const { width: W, height: H } = canvas;
const data = ctx.getImageData(0, 0, W, H).data;

const rgb = (x: number, y: number) => {
  const i = (y * W + x) * 4;
  return [data[i], data[i + 1], data[i + 2]] as const;
};
const lum = (x: number, y: number) => {
  const [r, g, b] = rgb(x, y);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};
const isRule = (x: number, y: number) => lum(x, y) < 150 && lum(x, y) > 40;
const isDark = (x: number, y: number) => lum(x, y) < 40;
const isCanvas = (x: number, y: number) => {
  const [r, g, b] = rgb(x, y);
  // semio-chrome-canvas ≈ 240,236,221
  return Math.abs(r - 240) < 18 && Math.abs(g - 236) < 18 && Math.abs(b - 221) < 22;
};
const isPhoto = (x: number, y: number) => {
  if (isCanvas(x, y) || isRule(x, y) || isDark(x, y)) return false;
  const [r, g, b] = rgb(x, y);
  // saturated / non-canvas content
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  return max - min > 25 || r > 180 && g < 160;
};

// Find table left border: densest rule column in left 15%
let borderX = 40;
let best = 0;
for (let x = 20; x < Math.floor(W * 0.15); x++) {
  let n = 0;
  for (let y = 0; y < H; y += 2) if (isRule(x, y)) n++;
  if (n > best) {
    best = n;
    borderX = x;
  }
}

// Find first strong full-width-ish horizontal rule in upper half (chip baseline)
const mid0 = Math.floor(W * 0.2);
const mid1 = Math.floor(W * 0.8);
const hRules: number[] = [];
for (let y = 40; y < Math.floor(H * 0.55); y++) {
  let n = 0;
  for (let x = mid0; x < mid1; x += 2) if (isRule(x, y)) n++;
  if (n > (mid1 - mid0) / 2 / 2 * 0.45) hRules.push(y);
}
const clusters: { y0: number; y1: number }[] = [];
for (const y of hRules) {
  const last = clusters[clusters.length - 1];
  if (last && y <= last.y1 + 2) last.y1 = y;
  else clusters.push({ y0: y, y1: y });
}

const results: Record<string, unknown> = {
  page: Number(pageArg),
  scale,
  borderX,
  hRuleClusters: clusters.slice(0, 8),
};

// For each of first 3 h-rules, measure pad below to photo / text
for (let i = 0; i < Math.min(3, clusters.length); i++) {
  const rule = clusters[i];
  const yAfter = rule.y1 + 1;
  // sample under left third (photo area) and mid (meta/text)
  let photoY = -1;
  let canvasRun = 0;
  for (let y = yAfter; y < Math.min(H, yAfter + 120); y++) {
    let photoHits = 0;
    let canvasHits = 0;
    for (let x = borderX + 8; x < borderX + 180; x++) {
      if (isPhoto(x, y)) photoHits++;
      if (isCanvas(x, y)) canvasHits++;
    }
    if (photoY < 0 && photoHits > 20) photoY = y;
    if (photoY < 0 && canvasHits > 100) canvasRun++;
  }
  const padPx = photoY < 0 ? -1 : photoY - yAfter;
  results[`rule${i}`] = {
    y0: rule.y0,
    y1: rule.y1,
    padPx,
    padPt: padPx < 0 ? null : +(padPx / scale).toFixed(2),
    canvasRowsBeforePhoto: canvasRun,
  };
}

// L-border join notches: bright pixels on borderX sandwiched by rule pixels
const notches: { y: number; L: number; rgb: number[] }[] = [];
for (let y = 80; y < H - 80; y++) {
  if (!isCanvas(borderX, y) && lum(borderX, y) > 170) {
    const above = [...Array(6)].some((_, k) => isRule(borderX, y - 1 - k));
    const below = [...Array(6)].some((_, k) => isRule(borderX, y + 1 + k));
    if (above && below) notches.push({ y, L: +lum(borderX, y).toFixed(1), rgb: [...rgb(borderX, y)] });
  }
}
// also: join where border is brighter than neighbors by gap pattern at hrule crossings
const joinGaps: { y: number; borderL: number; midL: number }[] = [];
for (const c of clusters) {
  for (let y = c.y0 - 1; y <= c.y1 + 1; y++) {
    if (y < 0 || y >= H) continue;
    const borderL = lum(borderX, y);
    const midL = lum(borderX + 3, y);
    // notch: border column lighter than a true rule while mid is rule
    if (borderL > 160 && midL < 150) joinGaps.push({ y, borderL: +borderL.toFixed(1), midL: +midL.toFixed(1) });
  }
}
results.notchCount = notches.length;
results.notchesSample = notches.slice(0, 12);
results.joinGapCount = joinGaps.length;
results.joinGapsSample = joinGaps.slice(0, 12);

// Crops
const mk = (name: string, x0: number, y0: number, w: number, h: number) => {
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
  const p = `${outPrefix}-${name}.png`;
  writeFileSync(p, c.toBuffer("image/png"));
  return p;
};
const r0 = clusters[0];
if (r0) {
  results.cropSeam = mk("seam", Math.max(0, borderX - 10), r0.y0 - 40, 520, 160);
  results.cropJoin = mk("join", Math.max(0, borderX - 6), r0.y0 + 80, 140, 220);
}
console.log(JSON.stringify(results, null, 2));
