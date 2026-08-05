#!/usr/bin/env bun
/** 🔬 [DEBUG] Locate phrase via text items, crop seam/joins, measure pad + notches. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, phrase, outPrefix] = process.argv.slice(2);
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
const scale = 6;
const viewport = page.getViewport({ scale });
const content = await page.getTextContent();
const matches: { str: string; x: number; y: number }[] = [];
for (const item of content.items as { str: string; transform: number[] }[]) {
  if (!item.str || !phrase.split(/\s+/).some((w) => item.str.includes(w))) continue;
  const [x, y] = viewport.convertToViewportPoint(item.transform[4], item.transform[5]);
  matches.push({ str: item.str, x, y });
}
matches.sort((a, b) => a.y - b.y || a.x - b.x);
console.log("[DEBUG] matches", matches.slice(0, 12));

const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await page.render({ canvasContext: ctx, viewport }).promise;
const W = canvas.width;
const H = canvas.height;
const data = ctx.getImageData(0, 0, W, H).data;
const lum = (x: number, y: number) => {
  const i = (y * W + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};
const rgb = (x: number, y: number) => {
  const i = (y * W + x) * 4;
  return [data[i], data[i + 1], data[i + 2]] as const;
};

const anchor = matches.find((m) => m.str.includes(phrase.slice(0, 6))) ?? matches[0];
if (!anchor) {
  console.log(JSON.stringify({ error: "no match", phrase, page: pageArg }));
  process.exit(1);
}

// Search downward from text for the chip baseline (long horizontal rule)
const xL = Math.max(20, Math.floor(anchor.x - 40));
const xR = Math.min(W - 20, Math.floor(anchor.x + 900));
let baselineY = -1;
for (let y = Math.floor(anchor.y) + 2; y < Math.min(H - 10, Math.floor(anchor.y) + 80); y++) {
  let n = 0;
  for (let x = xL; x < xR; x += 2) if (lum(x, y) < 150 && lum(x, y) > 50) n++;
  if (n > ((xR - xL) / 2) * 0.35) {
    baselineY = y;
    break;
  }
}

// Left border near table under baseline
let borderX = xL;
let best = 0;
const y0 = baselineY > 0 ? baselineY : Math.floor(anchor.y);
for (let x = Math.max(10, xL - 30); x < xL + 80; x++) {
  let n = 0;
  for (let y = y0; y < Math.min(H, y0 + 500); y++) if (lum(x, y) < 140 && lum(x, y) > 50) n++;
  if (n > best) {
    best = n;
    borderX = x;
  }
}

// Photo pad: from baselineY+1, count canvas-ish rows until non-canvas colorful
let photoY = -1;
let padPt: number | null = null;
if (baselineY > 0) {
  for (let y = baselineY + 1; y < baselineY + 100; y++) {
    let colorful = 0;
    for (let x = borderX + 10; x < borderX + 200; x++) {
      const [r, g, b] = rgb(x, y);
      const max = Math.max(r, g, b);
      const min = Math.min(r, g, b);
      if (max - min > 35) colorful++;
    }
    if (colorful > 25) {
      photoY = y;
      padPt = +((y - baselineY - 1) / scale).toFixed(2);
      break;
    }
  }
}

// Seam gap: page-cream rows under baseline before canvas/photo (lum very high + near page)
let creamGapPx = 0;
if (baselineY > 0) {
  for (let y = baselineY + 1; y < baselineY + 40; y++) {
    let pageish = 0;
    for (let x = borderX + 20; x < borderX + 200; x += 2) {
      const [r, g, b] = rgb(x, y);
      // page cream often same as canvas; detect by continuity with LEFT of border (margin)
      const margin = rgb(Math.max(0, borderX - 15), y);
      if (Math.abs(r - margin[0]) < 8 && Math.abs(g - margin[1]) < 8 && Math.abs(b - margin[2]) < 8) pageish++;
    }
    if (pageish > 60) creamGapPx++;
    else break;
  }
}

// Join notches on borderX below baseline
const notches: number[] = [];
if (baselineY > 0) {
  for (let y = baselineY + 20; y < Math.min(H - 5, baselineY + 900); y++) {
    const L = lum(borderX, y);
    if (L > 175) {
      const above = lum(borderX, y - 2) < 140;
      const below = lum(borderX, y + 2) < 140;
      if (above && below) notches.push(y);
    }
  }
}
const notchClusters: { y0: number; y1: number }[] = [];
for (const y of notches) {
  const last = notchClusters[notchClusters.length - 1];
  if (last && y <= last.y1 + 3) last.y1 = y;
  else notchClusters.push({ y0: y, y1: y });
}

const crop = (name: string, x0: number, y0: number, w: number, h: number) => {
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
  const p = `${outPrefix}-${name}.png`;
  writeFileSync(p, c.toBuffer("image/png"));
  return p;
};

const out = {
  page: Number(pageArg),
  phrase,
  anchor,
  baselineY,
  borderX,
  padPt,
  photoY,
  creamGapPx,
  creamGapPt: +(creamGapPx / scale).toFixed(2),
  notchClusters: notchClusters.length,
  notchesSample: notchClusters.slice(0, 15),
  cropSeam: baselineY > 0 ? crop("seam", Math.max(0, borderX - 8), baselineY - 50, 640, 140) : null,
  cropJoin: baselineY > 0 ? crop("join", Math.max(0, borderX - 4), baselineY + 40, 160, 280) : null,
  cropChip: crop("chip", Math.max(0, Math.floor(anchor.x) - 20), Math.max(0, Math.floor(anchor.y) - 30), 520, 100),
};
console.log(JSON.stringify(out, null, 2));
