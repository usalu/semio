#!/usr/bin/env bun
/** 📐 [DEBUG] temp: measure outer top-right corner gap of TOC Seite cell. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, outPng, yTopArg, yBotArg, x0Arg, x1Arg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas, loadImage } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const scale = 8;
const doc = await pdfjs
  .getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true })
  .promise;
const page = await doc.getPage(Number(pageArg));
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
const pageHpt = viewport.height / scale;
const yTop = Number(yTopArg);
const yBot = Number(yBotArg);
const x0 = Math.floor(Number(x0Arg) * scale);
const x1 = Math.ceil(Number(x1Arg) * scale);
const cy0 = Math.floor((pageHpt - yTop) * scale);
const cy1 = Math.ceil((pageHpt - yBot) * scale);
const y0 = Math.min(cy0, cy1);
const y1 = Math.max(cy0, cy1);
const crop = createCanvas(x1 - x0, y1 - y0);
crop.getContext("2d").drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, x1 - x0, y1 - y0);
writeFileSync(outPng, crop.toBuffer("image/png"));

const img = await loadImage(outPng);
const c2 = createCanvas(img.width, img.height);
const ctx = c2.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data, width, height } = ctx.getImageData(0, 0, img.width, img.height);
const lum = (x: number, y: number) => {
  const i = (y * width + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};
const isInk = (x: number, y: number) => lum(x, y) < 180;

let topY = -1;
for (let y = 0; y < height && topY < 0; y++) {
  for (let x = 0; x < width; x++) {
    if (isInk(x, y)) {
      topY = y;
      break;
    }
  }
}
let rightX = -1;
for (let x = width - 1; x >= 0 && rightX < 0; x--) {
  for (let y = 0; y < height; y++) {
    if (isInk(x, y)) {
      rightX = x;
      break;
    }
  }
}

const corner = [] as { x: number; y: number; lum: number; ink: boolean }[];
for (let dy = -2; dy <= 6; dy++) {
  for (let dx = -6; dx <= 2; dx++) {
    const x = rightX + dx;
    const y = topY + dy;
    if (x < 0 || y < 0 || x >= width || y >= height) continue;
    corner.push({ x, y, lum: Math.round(lum(x, y)), ink: isInk(x, y) });
  }
}

const notch =
  topY >= 0 &&
  rightX >= 0 &&
  !isInk(rightX, topY) &&
  (isInk(rightX - 1, topY) || isInk(rightX, topY + 1));

const result = {
  outPng,
  size: { width, height },
  topY,
  rightX,
  cornerPixelInk: isInk(rightX, topY),
  cornerLum: Math.round(lum(rightX, topY)),
  notchGap: notch,
  sample: corner.filter((p) => p.y === topY || p.x === rightX).slice(0, 40),
};
console.log(JSON.stringify(result, null, 2));
writeFileSync(outPng.replace(/\.png$/, ".json"), JSON.stringify(result, null, 2));
