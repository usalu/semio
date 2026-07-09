import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = "/Users/ueli/Documents/semio/.repo/🎫/26/07/09/ZWISCHENBERICHT-COVER-PAGE-LAYOUT";
const repoRoot = "/Users/ueli/Documents/semio";
const pdfPath = join(repoRoot, "mit-bestand/bericht/zwischenbericht/dist/verify-cover.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

console.log("Loading PDF...");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
console.log("Rendering page 1...");
const page = await doc.getPage(1);
const viewport = page.getViewport({ scale: 3 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
const out = join(ticketDir, "verify-cover-p1.png");
writeFileSync(out, canvas.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${out}`);
