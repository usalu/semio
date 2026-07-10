import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";

const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const pdfPath = process.argv[2];
const outPath = process.argv[3];
const pageNum = Number(process.argv[4] ?? "2");

const pdfBytes = readFileSync(pdfPath);
const doc = await pdfjs.getDocument({ data: new Uint8Array(pdfBytes), useSystemFonts: true }).promise;
const page = await doc.getPage(pageNum);
const viewport = page.getViewport({ scale: 3 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
await page.render({ canvas, canvasContext: context, viewport }).promise;
writeFileSync(outPath, canvas.toBuffer("image/png"));
console.log(`wrote ${outPath} (page ${pageNum}/${doc.numPages})`);
