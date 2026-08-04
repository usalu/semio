#!/usr/bin/env bun
/** 🖼️ [DEBUG] temp: rasterizes ticket probe PDFs so table rules can be eyeballed. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, outPrefix, pagesArg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const pages = pagesArg ? pagesArg.split(",").map(Number) : Array.from({ length: doc.numPages }, (_, i) => i + 1);
for (const number of pages) {
  const page = await doc.getPage(number);
  const viewport = page.getViewport({ scale: 200 / 72 });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
  writeFileSync(`${outPrefix}-${String(number).padStart(2, "0")}.png`, canvas.toBuffer("image/png"));
  console.log(`[DEBUG] rendered page ${number}`);
}
