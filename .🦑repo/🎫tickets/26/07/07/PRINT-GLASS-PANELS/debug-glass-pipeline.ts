import { createRequire } from "node:module";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = "/Users/ueli/Documents/semio";
const out = join(root, ".repo/🎫/26/07/07/PRINT-GLASS-PANELS/debug-out");
mkdirSync(out, { recursive: true });

const manifest = "panel-1;2;412.56499;471.78738;156.49014;230.99568";
const [, page, xPt, yPt, wPt, hPt] = manifest.split(";");
const entry = {
  page: Number.parseInt(page!, 10),
  xPt: Number.parseFloat(xPt!),
  yPt: Number.parseFloat(yPt!),
  wPt: Number.parseFloat(wPt!),
  hPt: Number.parseFloat(hPt!),
};

const PDF_PT_PER_INCH = 72;
const PANEL_RENDER_DPI = 200;
const renderScale = PANEL_RENDER_DPI / PDF_PT_PER_INCH;

const pdfPath = join(root, ".repo/🎫/26/07/07/PRINT-GLASS-PANELS/debug-out/pass1.pdf");
const pdfjsEntry = join(root, "node_modules/pdfjs-dist/legacy/build/pdf.mjs");
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const sharp = (await import("sharp")).default;

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const pg = await doc.getPage(entry.page);
const viewport = pg.getViewport({ scale: renderScale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d")!;
await pg.render({ canvas, canvasContext: context, viewport }).promise;
const pageWidthPt = (pg.view[2] ?? 0) - (pg.view[0] ?? 0);
const pageHeightPt = (pg.view[3] ?? 0) - (pg.view[1] ?? 0);

writeFileSync(join(out, "page-full.png"), canvas.toBuffer("image/png"));

const cropLeft = Math.max(0, Math.round(entry.xPt * renderScale));
const cropTop = Math.max(0, Math.round((pageHeightPt - entry.yPt - entry.hPt) * renderScale));
const cropWidth = Math.max(1, Math.round(entry.wPt * renderScale));
const cropHeight = Math.max(1, Math.round(entry.hPt * renderScale));

console.log("[DEBUG] pagePt", pageWidthPt, pageHeightPt);
console.log("[DEBUG] crop", { cropLeft, cropTop, cropWidth, cropHeight });

const fullPng = canvas.toBuffer("image/png");
const cropped = await sharp(fullPng).extract({ left: cropLeft, top: cropTop, width: cropWidth, height: cropHeight }).png().toBuffer();
writeFileSync(join(out, "crop-raw.png"), cropped);

const blurred = await sharp(cropped).blur(20).png().toBuffer();
writeFileSync(join(out, "crop-blur.png"), blurred);

const meta = await sharp(cropped).stats();
console.log("[DEBUG] crop stats", JSON.stringify(meta.channels.map((c) => ({ mean: c.mean, min: c.min, max: c.max }))));
