#!/usr/bin/env bun
/** 🔎 [DEBUG] Raster PDF page strip + score left-border continuity at joins. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, outPrefix, y0Arg, y1Arg, x0Arg, x1Arg] = process.argv.slice(2);
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
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await page.render({ canvasContext: ctx, viewport }).promise;
const img = ctx.getImageData(0, 0, canvas.width, canvas.height);
const data = img.data;
const lum = (x: number, y: number) => {
  const i = (y * canvas.width + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};

const y0 = Number(y0Arg ?? 0);
const y1 = Number(y1Arg ?? canvas.height - 1);
const x0 = Number(x0Arg ?? 0);
const x1 = Number(x1Arg ?? Math.min(canvas.width - 1, 80));

// Find left border x: first column with many dark pixels
let borderX = -1;
let best = 0;
for (let x = x0; x <= x1; x++) {
  let dark = 0;
  for (let y = y0; y <= y1; y++) if (lum(x, y) < 90) dark++;
  if (dark > best) {
    best = dark;
    borderX = x;
  }
}

const gaps: { y: number; L: number }[] = [];
const series: number[] = [];
for (let y = y0; y <= y1; y++) {
  const L = lum(borderX, y);
  series.push(L);
}
// A gap is a local bright run sandwiched between dark runs (join notch)
for (let y = 1; y < series.length - 1; y++) {
  const L = series[y];
  if (L > 140) {
    // look for dark within ±8px above and below
    const above = series.slice(Math.max(0, y - 8), y).some((v) => v < 100);
    const below = series.slice(y + 1, Math.min(series.length, y + 9)).some((v) => v < 100);
    if (above && below) gaps.push({ y: y0 + y, L });
  }
}

// Collapse contiguous gap rows
const clusters: { y0: number; y1: number; maxL: number }[] = [];
for (const g of gaps) {
  const last = clusters[clusters.length - 1];
  if (last && g.y <= last.y1 + 2) {
    last.y1 = g.y;
    last.maxL = Math.max(last.maxL, g.L);
  } else clusters.push({ y0: g.y, y1: g.y, maxL: g.L });
}

const cropW = Math.min(220, canvas.width - borderX + 20);
const cropH = y1 - y0 + 1;
const crop = createCanvas(cropW, cropH);
const cctx = crop.getContext("2d");
cctx.drawImage(canvas, borderX - 4, y0, cropW, cropH, 0, 0, cropW, cropH);
// mark gaps
cctx.fillStyle = "rgba(255,0,0,0.55)";
for (const c of clusters) {
  cctx.fillRect(0, c.y0 - y0, 8, c.y1 - c.y0 + 1);
}
const out = `${outPrefix}.png`;
writeFileSync(out, crop.toBuffer("image/png"));

console.log(
  JSON.stringify(
    {
      page: Number(pageArg),
      borderX,
      scan: { y0, y1, x0, x1 },
      gapClusters: clusters.length,
      clusters: clusters.slice(0, 30),
      out,
    },
    null,
    2,
  ),
);
