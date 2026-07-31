import { writeFileSync, existsSync } from "fs";
import { join } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const artifactDir = "/Users/ueli/.gemini/antigravity/brain/04fb2452-833d-4058-b616-fbc83e213820";

function loadPdfjsNapiCanvas() {
  const pdfjsEntry = fileURLToPath(new URL("../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
  return createRequire(pdfjsEntry)("@napi-rs/canvas");
}

async function renderPdf(pdfPath: string, pageNum: number, outName: string) {
  const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const { createCanvas } = loadPdfjsNapiCanvas();
  
  const pdfBytes = require("fs").readFileSync(pdfPath);
  const doc = await pdfjs.getDocument({ data: new Uint8Array(pdfBytes), useSystemFonts: true }).promise;
  
  const page = await doc.getPage(pageNum);
  const viewport = page.getViewport({ scale: 2 });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const context = canvas.getContext("2d");
  if (!context) throw new Error("canvas 2d unavailable");
  await page.render({ canvas, canvasContext: context, viewport }).promise;
  
  const outPath = join(artifactDir, outName);
  writeFileSync(outPath, canvas.toBuffer("image/png"));
  console.log(`Rendered page ${pageNum} to ${outPath}`);
}

async function main() {
  const pdfPath = "/Users/ueli/Documents/semio/print/dist/report.pdf";
  if (!existsSync(pdfPath)) {
    console.error("PDF not found:", pdfPath);
    return;
  }
  // Let's render first 3 pages
  try {
    await renderPdf(pdfPath, 1, "page1.png");
    await renderPdf(pdfPath, 2, "page2.png");
    await renderPdf(pdfPath, 3, "page3.png");
  } catch (err) {
    console.error(err);
  }
}

main();
