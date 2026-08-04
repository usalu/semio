#!/usr/bin/env bun
/** 📏 [DEBUG] temp: measures vertical gap from hairline rows to nearest text on a cropped band. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, outPng, yTopArg, yBotArg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(Number(pageArg));
const scale = 4;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await page.render({ canvasContext: ctx, viewport }).promise;
const img = ctx.getImageData(0, 0, canvas.width, canvas.height);
const pageH = viewport.height;
const yTop = Number(yTopArg);
const yBot = Number(yBotArg);
const y0 = Math.floor((pageH / 72) * scale - (yTop * scale));
const y1 = Math.ceil((pageH / 72) * scale - (yBot * scale));
const x0 = Math.floor(60 * scale);
const x1 = Math.ceil(520 * scale);

function isInk(r: number, g: number, b: number): boolean {
  // Dark theme: light text on dark bg. Light theme: dark text on cream.
  const lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
  return lum > 140 || lum < 90;
}
function isRule(r: number, g: number, b: number, a: number): boolean {
  if (a < 200) return false;
  const lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
  // mid-grey hairlines (neither cream ~230 nor near-black text ~40)
  return lum > 95 && lum < 175;
}

type Band = { y: number; kind: "rule" | "ink"; count: number };
const bands: Band[] = [];
for (let y = Math.min(y0, y1); y <= Math.max(y0, y1); y++) {
  let rule = 0;
  let ink = 0;
  for (let x = x0; x < x1; x++) {
    const i = (y * canvas.width + x) * 4;
    const r = img.data[i], g = img.data[i + 1], b = img.data[i + 2], a = img.data[i + 3];
    if (isRule(r, g, b, a)) rule++;
    else if (isInk(r, g, b)) ink++;
  }
  const span = x1 - x0;
  if (rule > span * 0.35) bands.push({ y, kind: "rule", count: rule });
  else if (ink > 8) bands.push({ y, kind: "ink", count: ink });
}

// Collapse contiguous bands
const collapsed: { y0: number; y1: number; kind: "rule" | "ink" }[] = [];
for (const b of bands) {
  const last = collapsed[collapsed.length - 1];
  if (last && last.kind === b.kind && b.y === last.y1 + 1) last.y1 = b.y;
  else collapsed.push({ y0: b.y, y1: b.y, kind: b.kind });
}

const toPt = (py: number) => (pageH - py / scale) / (scale / scale); // keep in raster px → pt via /scale then flip
const pxToPt = (px: number) => px / scale;

console.log(`[DEBUG] page ${pageArg} crop y=${yTop}..${yBot}pt → raster ${Math.min(y0,y1)}..${Math.max(y0,y1)}`);
for (let i = 0; i < collapsed.length; i++) {
  const c = collapsed[i];
  const mid = (c.y0 + c.y1) / 2;
  const h = c.y1 - c.y0 + 1;
  let gap = "";
  if (i > 0) {
    const prev = collapsed[i - 1];
    gap = `  gap-from-prev=${pxToPt(c.y0 - prev.y1 - 1).toFixed(2)}pt`;
  }
  console.log(`[DEBUG] ${c.kind} ypx=${c.y0}-${c.y1} h=${h}px (${pxToPt(h).toFixed(2)}pt)${gap}`);
}

// Write crop
const cy0 = Math.min(y0, y1);
const ch = Math.abs(y1 - y0) + 1;
const crop = createCanvas(x1 - x0, ch);
crop.getContext("2d").drawImage(canvas, x0, cy0, x1 - x0, ch, 0, 0, x1 - x0, ch);
writeFileSync(outPng, crop.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${outPng}`);
