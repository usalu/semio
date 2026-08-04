#!/usr/bin/env bun
/** 🔎 [DEBUG] temp: dump per-row luminance stats under left chip for seam diagnosis. */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const [pngPath, y0Arg, y1Arg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const x0 = Math.floor(img.width * 0.10);
const x1 = Math.floor(img.width * 0.30);
const y0 = Number(y0Arg ?? 0);
const y1 = Number(y1Arg ?? img.height - 1);

function lum(i: number) {
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

console.log(`[DEBUG] ${pngPath} rows ${y0}..${y1} x=${x0}..${x1}`);
for (let y = y0; y <= y1; y++) {
  let min = 255, max = 0, sum = 0, n = 0;
  let dark = 0, mid = 0, light = 0;
  for (let x = x0; x < x1; x++) {
    const L = lum((y * img.width + x) * 4);
    min = Math.min(min, L);
    max = Math.max(max, L);
    sum += L;
    n++;
    if (L < 80) dark++;
    else if (L < 190) mid++;
    else light++;
  }
  const avg = sum / n;
  const tag = mid > n * 0.35 ? "RULE" : dark > 8 ? "INK " : light > n * 0.5 ? "PAGE" : "FILL";
  if (tag === "RULE" || tag === "PAGE" || (y % 4 === 0)) {
    console.log(
      `[DEBUG] y=${String(y).padStart(3)} ${tag} avg=${avg.toFixed(1)} min=${min.toFixed(0)} mid%=${((mid / n) * 100).toFixed(0)} light%=${((light / n) * 100).toFixed(0)}`,
    );
  }
}
