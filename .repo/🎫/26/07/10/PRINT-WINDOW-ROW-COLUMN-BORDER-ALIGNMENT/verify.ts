#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildPrintDocument } from "../../../../../../print/script.ts";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const distDir = join(ticketDir, "dist");
const texAbs = join(ticketDir, "verify-window-alignment.tex");
const zwischenberichtPdf = join(repoRoot, "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf");

await buildPrintDocument(texAbs, distDir);

const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const jobs: readonly (readonly [string, number, string])[] = [
  [join(distDir, "verify-window-alignment.pdf"), 1, "verify-window-alignment-p1"],
  [zwischenberichtPdf, 1, "zwischenbericht-p1-cover"],
];

for (const [pdfPath, pageNum, outStem] of jobs) {
  const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
  const page = await doc.getPage(pageNum);
  const viewport = page.getViewport({ scale: 3 });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const context = canvas.getContext("2d");
  if (!context) throw new Error("canvas 2d unavailable");
  await page.render({ canvas, canvasContext: context, viewport }).promise;
  const out = join(ticketDir, `${outStem}.png`);
  writeFileSync(out, canvas.toBuffer("image/png"));
  console.log(`[DEBUG] wrote ${out}`);
}
