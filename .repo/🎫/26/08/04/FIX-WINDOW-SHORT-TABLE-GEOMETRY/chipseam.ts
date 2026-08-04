#!/usr/bin/env bun
/** 📏 [DEBUG] temp: scan under left chip for stacked hairlines + page gap. */
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

function lum(i: number) {
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

// Under the left chip body (avoid left vertical + mid gap).
const x0 = Math.floor(img.width * 0.08);
const x1 = Math.floor(img.width * 0.28);

type Kind = "rule" | "page" | "canvas" | "ink";
const kinds: Kind[] = [];
for (let y = 0; y < img.height; y++) {
  let rule = 0, page = 0, canvasN = 0, ink = 0;
  for (let x = x0; x < x1; x++) {
    const L = lum((y * img.width + x) * 4);
    if (L > 95 && L < 185) rule++;
    else if (L > 215) page++;
    else if (L < 40) ink++;
    else canvasN++;
  }
  const span = x1 - x0;
  if (rule > span * 0.4) kinds.push("rule");
  else if (page > span * 0.4) kinds.push("page");
  else if (ink > 10) kinds.push("ink");
  else kinds.push("canvas");
}

const bands: { kind: Kind; y0: number; y1: number }[] = [];
for (let y = 0; y < kinds.length; y++) {
  const last = bands[bands.length - 1];
  if (last && last.kind === kinds[y] && y === last.y1 + 1) last.y1 = y;
  else bands.push({ kind: kinds[y], y0: y, y1: y });
}

console.log(`[DEBUG] ${pngPath} under-left-chip x=${x0}..${x1}`);
for (const b of bands) {
  const h = (b.y1 - b.y0 + 1) / scale;
  if (b.kind === "page" || b.kind === "rule" || (b.kind === "canvas" && h < 2)) {
    console.log(`[DEBUG] ${b.kind.padEnd(6)} y=${b.y0}-${b.y1} h=${h.toFixed(2)}pt`);
  }
}

// Find first rule below chip text, then next rule — that's the seam pair.
let seenInk = false;
const seamRules: { y0: number; y1: number }[] = [];
for (const b of bands) {
  if (b.kind === "ink") seenInk = true;
  if (!seenInk) continue;
  if (b.kind === "rule") seamRules.push({ y0: b.y0, y1: b.y1 });
  if (seamRules.length >= 2) break;
}
if (seamRules.length >= 2) {
  const a = seamRules[0], b = seamRules[1];
  let pagePx = 0;
  for (let y = a.y1 + 1; y < b.y0; y++) if (kinds[y] === "page") pagePx++;
  const gapPx = b.y0 - a.y1 - 1;
  console.log(
    `[DEBUG] SEAM pair ${a.y0}-${a.y1} → ${b.y0}-${b.y1}: gap=${(gapPx / scale).toFixed(2)}pt page=${(pagePx / scale).toFixed(2)}pt ${gapPx <= 0 ? "WELDED" : "DOUBLE+GAP"}`,
  );
} else if (seamRules.length === 1) {
  console.log(`[DEBUG] SEAM single rule ${seamRules[0].y0}-${seamRules[0].y1}: WELDED (no second hairline)`);
} else {
  console.log(`[DEBUG] SEAM no rule found under chip text`);
}
