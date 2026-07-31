#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const canvasMod = createRequire(pdfjsEntry)("@napi-rs/canvas");
globalThis.DOMMatrix = canvasMod.DOMMatrix;
globalThis.ImageData = canvasMod.ImageData;
globalThis.Path2D = canvasMod.Path2D;
const { createCanvas } = canvasMod;
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const scale = 4;
const pdfPath = join(ticketDir, "dist/probe-outline-border.pdf");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const lines: string[] = ["# Outline Chip Border Verify", ""];

for (let pageNum = 1; pageNum <= doc.numPages; pageNum++) {
  const page = await doc.getPage(pageNum);
  const viewport = page.getViewport({ scale });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const context = canvas.getContext("2d");
  if (!context) throw new Error("canvas 2d unavailable");
  await page.render({ canvas, canvasContext: context, viewport }).promise;
  const out = join(ticketDir, `probe-p${pageNum}.png`);
  writeFileSync(out, canvas.toBuffer("image/png"));
  lines.push(`## page ${pageNum}`);
  lines.push(`- raster: probe-p${pageNum}.png`);
  lines.push("");
  console.log(`[DEBUG] rasterized page ${pageNum} ${Math.ceil(viewport.width)}x${Math.ceil(viewport.height)}`);
}

writeFileSync(join(ticketDir, "verify-log.md"), lines.join("\n"));
