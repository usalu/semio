#!/usr/bin/env bun
/** 📐 [DEBUG] temp: dump luminance grid at table outer top-left/right corners. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, side, outPng, theme = "light"] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const scale = 12;
const doc = await pdfjs
  .getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true })
  .promise;
const page = await doc.getPage(Number(pageArg));
const tc = await page.getTextContent();
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
const ctx = canvas.getContext("2d");

const seite = (tc.items as { str: string; transform: number[] }[]).find((i) => i.str === "Seite");
if (!seite) throw new Error("Seite not found");
const sx = seite.transform[4];
const sy = seite.transform[5];

// Approximate table outer edges from known TOC geometry.
const rightXpt = sx + 35;
const leftXpt = 56.5;
const topYpt = sy + 14;

const cx = side === "L" ? leftXpt : rightXpt;
const cy = topYpt;
const half = 8;
const x0 = Math.floor((cx - half) * scale);
const x1 = Math.ceil((cx + half) * scale);
const pageH = viewport.height / scale;
const y0 = Math.floor((pageH - (cy + half)) * scale);
const y1 = Math.ceil((pageH - (cy - half)) * scale);
const w = x1 - x0;
const h = y1 - y0;
const crop = createCanvas(w, h);
crop.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
writeFileSync(outPng, crop.toBuffer("image/png"));

const data = crop.getContext("2d").getImageData(0, 0, w, h).data;
const lum = (x: number, y: number) => {
  const i = (y * w + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};

const inkThresh = theme === "dark" ? 80 : 180;
const isInk = (x: number, y: number) =>
  theme === "dark" ? lum(x, y) > inkThresh : lum(x, y) < inkThresh;

// Find topmost ink row and leftmost/rightmost ink on that band.
let topY = -1;
for (let y = 0; y < h && topY < 0; y++) {
  for (let x = 0; x < w; x++) {
    if (isInk(x, y)) {
      topY = y;
      break;
    }
  }
}
let edgeX = side === "L" ? w : -1;
if (side === "L") {
  for (let x = 0; x < w; x++) {
    for (let y = 0; y < h; y++) {
      if (isInk(x, y)) {
        edgeX = x;
        x = w;
        break;
      }
    }
  }
} else {
  for (let x = w - 1; x >= 0; x--) {
    for (let y = 0; y < h; y++) {
      if (isInk(x, y)) {
        edgeX = x;
        x = -1;
        break;
      }
    }
  }
}

const grid: number[][] = [];
for (let dy = 0; dy < 10; dy++) {
  const row: number[] = [];
  for (let dx = -5; dx <= 5; dx++) {
    const x = edgeX + dx;
    const y = topY + dy;
    row.push(x >= 0 && y >= 0 && x < w && y < h ? Math.round(lum(x, y)) : -1);
  }
  grid.push(row);
}

const cornerInk = isInk(edgeX, topY);
const result = {
  side,
  theme,
  topY,
  edgeX,
  cornerInk,
  cornerLum: Math.round(lum(edgeX, topY)),
  grid,
};
console.log(JSON.stringify(result, null, 2));
writeFileSync(outPng.replace(/\.png$/, ".json"), JSON.stringify(result, null, 2));
