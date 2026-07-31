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

const img = await loadImage(join(ticketDir, "probe-p2.png"));
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
if (!ctx) throw new Error("no 2d");
ctx.drawImage(img, 0, 0);
const { data, width, height } = ctx.getImageData(0, 0, img.width, img.height);

function lum(x: number, y: number): number {
  const i = (y * width + x) * 4;
  return data[i]! + data[i + 1]! + data[i + 2]!;
}
function isRed(x: number, y: number): boolean {
  if (x < 0 || y < 0 || x >= width || y >= height) return false;
  const i = (y * width + x) * 4;
  return data[i]! > 140 && data[i + 1]! < 120 && data[i + 2]! < 140 && data[i]! > data[i + 1]! + 40;
}

// Find Bauteilportal title chip: scan for red outline rows in upper half
const portalBands: Array<{ y0: number; y1: number }> = [];
for (let y = 200; y < height / 2; y++) {
  let c = 0;
  for (let x = 60; x < 900; x++) if (isRed(x, y)) c++;
  if (c > 30) {
    const last = portalBands[portalBands.length - 1];
    if (last && y <= last.y1 + 3) last.y1 = y;
    else portalBands.push({ y0: y, y1: y });
  }
}
const bands = portalBands.filter((b) => b.y1 - b.y0 >= 10 && b.y1 - b.y0 <= 60);
console.log("[DEBUG] bands", bands);

for (const [idx, band] of bands.entries()) {
  // leftmost red = title chip left; find title chip right vertical before gap
  let left = -1;
  let titleRight = -1;
  const yMid = Math.round((band.y0 + band.y1) / 2);
  for (let x = 40; x < width - 40; x++) if (isRed(x, yMid)) { left = x; break; }
  // title chip right: first gap in red run after left
  let inChip = false;
  let runEnd = left;
  for (let x = left; x < width - 40; x++) {
    if (isRed(x, yMid)) {
      inChip = true;
      runEnd = x;
    } else if (inChip && x - runEnd > 8) {
      titleRight = runEnd;
      break;
    }
  }
  // far right chip outer
  let farRight = -1;
  for (let x = width - 40; x > left; x--) if (isRed(x, yMid)) { farRight = x; break; }

  // At titleRight: compare bottom-most red vs mid vertical
  let botAtRight = -1;
  for (let y = band.y1 + 8; y >= band.y0; y--) {
    if (isRed(titleRight, y) || isRed(titleRight - 1, y) || isRed(titleRight + 1, y)) {
      botAtRight = y;
      break;
    }
  }
  // bottom rule y: scan full width red near band.y1
  let botRuleY = -1;
  for (let y = band.y1 - 2; y <= band.y1 + 6; y++) {
    let c = 0;
    for (let x = left; x < farRight; x++) if (isRed(x, y)) c++;
    if (c > (farRight - left) * 0.5) { botRuleY = y; break; }
  }
  // left vertical bottom vs bot rule
  let leftBot = -1;
  for (let y = band.y1 + 8; y >= band.y0; y--) {
    if (isRed(left, y) || isRed(left + 1, y)) { leftBot = y; break; }
  }

  console.log(`[DEBUG] band ${idx} left=${left} titleRight=${titleRight} farRight=${farRight} leftBot=${leftBot} botRuleY=${botRuleY} botAtRight=${botAtRight} leftDelta=${leftBot - botRuleY} rightEdgeDelta=${farRight}`);

  // crops: title chip bottom-left, bottom-right, far-right chip
  for (const [name, cx, cy] of [
    ["portal-bl", left - 5, band.y1 - 25],
    ["portal-br-title", titleRight - 40, band.y1 - 25],
    ["portal-br-num", farRight - 80, band.y1 - 25],
  ] as const) {
    const crop = createCanvas(90, 50);
    const cctx = crop.getContext("2d");
    if (!cctx) continue;
    cctx.drawImage(canvas, cx, cy, 90, 50, 0, 0, 90, 50);
    writeFileSync(join(ticketDir, `${name}-${idx}.png`), crop.toBuffer("image/png"));
  }
}
