#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const pdfPath = join(repoRoot, "print/dist/paper.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const chromeBase = { r: 247, g: 243, b: 227 };
const canvasBg = { r: 240, g: 236, b: 221 };

function chromeLeftEdge(data: Uint8ClampedArray, width: number, y: number): number {
  for (let x = 0; x < width; x++) {
    const i = (y * width + x) * 4;
    const dr = Math.abs(data[i] - chromeBase.r);
    const dg = Math.abs(data[i + 1] - chromeBase.g);
    const db = Math.abs(data[i + 2] - chromeBase.b);
    if (dr + dg + db < 30) return x;
  }
  return -1;
}

function chromeBand(data: Uint8ClampedArray, width: number, height: number, yStart: number, yEnd: number): number {
  for (let y = yStart; y < yEnd; y++) {
    let chromePixels = 0;
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4;
      const dr = Math.abs(data[i] - chromeBase.r);
      const dg = Math.abs(data[i + 1] - chromeBase.g);
      const db = Math.abs(data[i + 2] - chromeBase.b);
      if (dr + dg + db < 30) chromePixels++;
    }
    if (chromePixels > width * 0.5) return y;
  }
  return -1;
}

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(2);
const viewport = page.getViewport({ scale: 2 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
const { data, width, height } = context.getImageData(0, 0, canvas.width, canvas.height);

const navbarY = chromeBand(data, width, height, 0, Math.floor(height * 0.2));
const footerY = chromeBand(data, width, height, Math.floor(height * 0.75), height);
const navbarLeft = navbarY >= 0 ? chromeLeftEdge(data, width, navbarY) : -1;
const footerLeft = footerY >= 0 ? chromeLeftEdge(data, width, footerY) : -1;
const footerBottom = (() => {
  for (let y = height - 1; y >= Math.floor(height * 0.75); y--) {
    let chromePixels = 0;
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4;
      const dr = Math.abs(data[i] - chromeBase.r);
      const dg = Math.abs(data[i + 1] - chromeBase.g);
      const db = Math.abs(data[i + 2] - chromeBase.b);
      if (dr + dg + db < 30) chromePixels++;
    }
    if (chromePixels > width * 0.5) return y;
  }
  return -1;
})();

let contentBottom = -1;
for (let y = footerY - 1; y >= 0; y--) {
  let contentPixels = 0;
  for (let x = Math.floor(width * 0.1); x < Math.floor(width * 0.9); x++) {
    const i = (y * width + x) * 4;
    const dr = Math.abs(data[i] - canvasBg.r);
    const dg = Math.abs(data[i + 1] - canvasBg.g);
    const db = Math.abs(data[i + 2] - canvasBg.b);
    if (dr + dg + db > 25) contentPixels++;
  }
  if (contentPixels > width * 0.05) {
    contentBottom = y;
    break;
  }
}

const pxPerPt = height / 842;
const alignDelta = Math.abs(navbarLeft - footerLeft);
console.log(`[DEBUG] page2 navbar y=${navbarY} left=${navbarLeft}px footer y=${footerY} left=${footerLeft}px align delta=${alignDelta}px (${(alignDelta / pxPerPt).toFixed(2)}pt)`);
console.log(`[DEBUG] footer bottom=${footerBottom}px gap below footer=${height - 1 - footerBottom}px content→footer=${footerY - contentBottom - 1}px`);
