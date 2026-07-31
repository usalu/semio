#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const canvasMod = createRequire(pdfjsEntry)("@napi-rs/canvas");
globalThis.DOMMatrix = canvasMod.DOMMatrix;
globalThis.ImageData = canvasMod.ImageData;
globalThis.Path2D = canvasMod.Path2D;
const { createCanvas, loadImage } = canvasMod;

const pngPath = join(ticketDir, "probe-p3.png");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
if (!ctx) throw new Error("no 2d");
ctx.drawImage(img, 0, 0);
const { data, width, height } = ctx.getImageData(0, 0, img.width, img.height);

function dark(x: number, y: number): boolean {
  if (x < 0 || y < 0 || x >= width || y >= height) return false;
  const i = (y * width + x) * 4;
  return data[i]! + data[i + 1]! + data[i + 2]! < 500;
}

// Scan for the paragraph title chip row by finding the distinctive wide outline near mid-page.
const rows: number[] = [];
for (let y = 0; y < height; y++) {
  let darkCount = 0;
  for (let x = 80; x < width - 80; x++) if (dark(x, y)) darkCount++;
  if (darkCount > 40 && darkCount < width * 0.55) rows.push(y);
}

const bands: Array<{ y0: number; y1: number }> = [];
for (const y of rows) {
  const last = bands[bands.length - 1];
  if (last && y <= last.y1 + 2) last.y1 = y;
  else bands.push({ y0: y, y1: y });
}

const headingBands = bands.filter((b) => b.y1 - b.y0 >= 8 && b.y1 - b.y0 <= 80);
const lines: string[] = ["# Pixel edge audit", "", `bands: ${headingBands.length}`, ""];

for (const [idx, band] of headingBands.entries()) {
  const yMid = Math.round((band.y0 + band.y1) / 2);
  const yBot = band.y1;
  // Find rightmost dark pixel on bottom stroke row and on mid (vertical) row
  let rightMid = -1;
  let rightBot = -1;
  for (let x = width - 1; x >= 0; x--) {
    if (rightMid < 0 && dark(x, yMid)) rightMid = x;
    if (rightBot < 0 && dark(x, yBot)) rightBot = x;
    if (rightMid >= 0 && rightBot >= 0) break;
  }
  const delta = rightBot - rightMid;
  lines.push(`## band ${idx} y=${band.y0}-${band.y1}`);
  lines.push(`- rightMid=${rightMid} rightBot=${rightBot} delta=${delta}`);
  lines.push("");

  // Crop right chip corner for visual check
  const cropW = 120;
  const cropH = band.y1 - band.y0 + 20;
  const cropX = Math.max(0, rightMid - cropW + 20);
  const cropY = Math.max(0, band.y0 - 10);
  const crop = createCanvas(cropW, cropH);
  const cctx = crop.getContext("2d");
  if (!cctx) continue;
  cctx.drawImage(canvas, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH);
  const out = join(ticketDir, `corner-band-${idx}.png`);
  writeFileSync(out, crop.toBuffer("image/png"));
  console.log(`[DEBUG] band ${idx} delta=${delta} crop=${out}`);
}

writeFileSync(join(ticketDir, "pixel-audit.md"), lines.join("\n"));
console.log(lines.join("\n"));
