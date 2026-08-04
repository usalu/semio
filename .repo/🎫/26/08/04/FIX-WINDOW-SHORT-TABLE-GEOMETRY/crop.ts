#!/usr/bin/env bun
/** ✂️ [DEBUG] temp: crop a PDF page band to PNG for close visual QA. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, outPng, yTopArg, yBotArg, x0Arg, x1Arg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(Number(pageArg));
const scale = 4;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
const pageHpt = viewport.height / scale;
const yTop = Number(yTopArg);
const yBot = Number(yBotArg);
const x0 = Math.floor(Number(x0Arg ?? 60) * scale);
const x1 = Math.ceil(Number(x1Arg ?? 540) * scale);
const cy0 = Math.floor((pageHpt - yTop) * scale);
const cy1 = Math.ceil((pageHpt - yBot) * scale);
const y0 = Math.min(cy0, cy1);
const y1 = Math.max(cy0, cy1);
const crop = createCanvas(x1 - x0, y1 - y0);
crop.getContext("2d").drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, x1 - x0, y1 - y0);
writeFileSync(outPng, crop.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${outPng} (${x1 - x0}x${y1 - y0})`);
