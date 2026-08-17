#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const pdfPath = "/Users/ueli/Documents/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
console.log(`[DEBUG] pages: ${doc.numPages}`);
for (const pageNum of [doc.numPages - 1, doc.numPages]) {
  const page = await doc.getPage(pageNum);
  const viewport = page.getViewport({ scale: 3 });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const context = canvas.getContext("2d");
  await page.render({ canvas, canvasContext: context, viewport }).promise;
  const out = join(ticketDir, `zwischenbericht-p${pageNum}.png`);
  writeFileSync(out, canvas.toBuffer("image/png"));
  console.log(`[DEBUG] wrote ${out}`);
}
