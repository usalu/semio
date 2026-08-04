#!/usr/bin/env bun
/** 🔬 [DEBUG] temp: scan left border column for page/cream gaps at mid-rule joins. */
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

function rgb(x: number, y: number) {
  const i = (y * img.width + x) * 4;
  return { r: data[i], g: data[i + 1], b: data[i + 2], L: 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2] };
}

// Find leftmost border-ish column (mid-grey rule ink over many rows)
let borderX = -1;
let best = 0;
for (let x = 0; x < Math.min(img.width, 120); x++) {
  let rule = 0;
  for (let y = 0; y < img.height; y++) {
    const { L } = rgb(x, y);
    if (L > 70 && L < 190) rule++;
  }
  if (rule > best) {
    best = rule;
    borderX = x;
  }
}
console.log(`[DEBUG] ${pngPath} bestBorderX=${borderX} ruleRows=${best}/${img.height}`);

type Kind = "rule" | "page" | "canvas" | "other";
const kinds: Kind[] = [];
for (let y = 0; y < img.height; y++) {
  // sample a 3px-wide ribbon
  let rule = 0, page = 0, canvasN = 0;
  for (let dx = -1; dx <= 1; dx++) {
    const x = Math.max(0, Math.min(img.width - 1, borderX + dx));
    const { L, r, g, b } = rgb(x, y);
    if (L > 70 && L < 190) rule++;
    else if (L > 230 || (r > 240 && g > 235 && b > 220)) page++;
    else if (L > 200 && L <= 230) canvasN++;
    else if (L < 55) canvasN++; // dark canvas
    else canvasN++;
  }
  if (rule >= 2) kinds.push("rule");
  else if (page >= 2) kinds.push("page");
  else kinds.push(canvasN >= 2 ? "canvas" : "other");
}

const bands: { kind: Kind; y0: number; y1: number }[] = [];
for (let y = 0; y < kinds.length; y++) {
  const last = bands[bands.length - 1];
  if (last && last.kind === kinds[y] && y === last.y1 + 1) last.y1 = y;
  else bands.push({ kind: kinds[y], y0: y, y1: y });
}

let gaps = 0;
let joins = 0;
for (let i = 1; i < bands.length - 1; i++) {
  const b = bands[i];
  const h = (b.y1 - b.y0 + 1) / scale;
  if (bands[i - 1].kind === "rule" && bands[i + 1].kind === "rule") {
    joins++;
    if (b.kind !== "rule" && h >= 0.2) {
      gaps++;
      console.log(`[DEBUG] JOIN GAP kind=${b.kind} h=${h.toFixed(2)}pt y=${b.y0}-${b.y1}`);
    }
  }
}
console.log(`[DEBUG] joins≈${joins} cream/page gaps=${gaps}`);
// dump short non-rule bands for inspection
for (const b of bands) {
  const h = (b.y1 - b.y0 + 1) / scale;
  if (b.kind !== "rule" && h <= 3) console.log(`[DEBUG] short ${b.kind} h=${h.toFixed(2)}pt y=${b.y0}-${b.y1}`);
}
