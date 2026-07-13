#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const pdfPath = join(repoRoot, "print/dist/zwischenbericht.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const chromeBase = { r: 247, g: 243, b: 227 };
const canvasBg = { r: 240, g: 236, b: 221 };

function isChrome(data: Uint8ClampedArray, i: number): boolean {
  const dr = Math.abs(data[i] - chromeBase.r);
  const dg = Math.abs(data[i + 1] - chromeBase.g);
  const db = Math.abs(data[i + 2] - chromeBase.b);
  return dr + dg + db < 30;
}
function isCanvasBg(data: Uint8ClampedArray, i: number): boolean {
  const dr = Math.abs(data[i] - canvasBg.r);
  const dg = Math.abs(data[i + 1] - canvasBg.g);
  const db = Math.abs(data[i + 2] - canvasBg.b);
  return dr + dg + db < 15;
}

function chromeRowFrac(data: Uint8ClampedArray, width: number, y: number): number {
  let n = 0;
  for (let x = 0; x < width; x++) if (isChrome(data, (y * width + x) * 4)) n++;
  return n / width;
}

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(2);
const viewport = page.getViewport({ scale: 3 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
writeFileSync(join(ticketDir, "zwischenbericht-p2.png"), canvas.toBuffer("image/png"));
const { data, width, height } = context.getImageData(0, 0, canvas.width, canvas.height);

// header band: top region
let headerBarTop = -1;
let headerBarBottom = -1;
for (let y = 0; y < Math.floor(height * 0.15); y++) {
  const frac = chromeRowFrac(data, width, y);
  if (frac > 0.5 && headerBarTop < 0) headerBarTop = y;
  if (frac > 0.5) headerBarBottom = y;
}
let bodyTextTop = -1;
for (let y = headerBarBottom + 1; y < Math.floor(height * 0.3); y++) {
  let contentPixels = 0;
  for (let x = Math.floor(width * 0.1); x < Math.floor(width * 0.9); x++) {
    if (!isCanvasBg(data, (y * width + x) * 4) && !isChrome(data, (y * width + x) * 4)) contentPixels++;
  }
  if (contentPixels > width * 0.02) {
    bodyTextTop = y;
    break;
  }
}

// footer band: bottom region
let footerBarTop = -1;
let footerBarBottom = -1;
for (let y = height - 1; y > Math.floor(height * 0.85); y--) {
  const frac = chromeRowFrac(data, width, y);
  if (frac > 0.5 && footerBarBottom < 0) footerBarBottom = y;
  if (frac > 0.5) footerBarTop = y;
}
let bodyTextBottom = -1;
for (let y = footerBarTop - 1; y > Math.floor(height * 0.7); y--) {
  let contentPixels = 0;
  for (let x = Math.floor(width * 0.1); x < Math.floor(width * 0.9); x++) {
    if (!isCanvasBg(data, (y * width + x) * 4) && !isChrome(data, (y * width + x) * 4)) contentPixels++;
  }
  if (contentPixels > width * 0.02) {
    bodyTextBottom = y;
    break;
  }
}

const scale = viewport.scale; // px per pt
console.log(`[DEBUG] page px size: ${width}x${height} at scale=${scale}`);
console.log(`[DEBUG] header bar: top=${headerBarTop} bottom=${headerBarBottom} (height=${headerBarBottom - headerBarTop}px = ${((headerBarBottom - headerBarTop) / scale).toFixed(2)}pt)`);
console.log(`[DEBUG] body text starts at y=${bodyTextTop}`);
console.log(`[DEBUG] HEADER gap (bar-bottom -> body-text-top) = ${bodyTextTop - headerBarBottom}px = ${((bodyTextTop - headerBarBottom) / scale).toFixed(2)}pt`);
console.log(`[DEBUG] footer bar: top=${footerBarTop} bottom=${footerBarBottom} (height=${footerBarBottom - footerBarTop}px = ${((footerBarBottom - footerBarTop) / scale).toFixed(2)}pt)`);
console.log(`[DEBUG] body text ends at y=${bodyTextBottom}`);
console.log(`[DEBUG] FOOTER gap (body-text-bottom -> bar-top) = ${footerBarTop - bodyTextBottom}px = ${((footerBarTop - bodyTextBottom) / scale).toFixed(2)}pt`);
