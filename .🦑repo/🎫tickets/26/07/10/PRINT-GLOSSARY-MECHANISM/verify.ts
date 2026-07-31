#!/usr/bin/env bun
import { copyFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildPrintDocument } from "../../../../../../print/📜script.ts";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const printRoot = join(repoRoot, "print");
const texSource = join(ticketDir, "verify-glossary.tex");
const texAbs = join(printRoot, "dist/verify-glossary.tex");
const outDir = join(ticketDir, "dist");

copyFileSync(texSource, texAbs);
await buildPrintDocument(texAbs, outDir);

const glossaryTex = join(outDir, "verify-glossary.glossary.tex");
if (existsSync(glossaryTex)) copyFileSync(glossaryTex, join(ticketDir, "verify-glossary.glossary.tex"));

const pdfPath = join(outDir, "verify-glossary.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
for (const pageNum of [1, doc.numPages]) {
  const page = await doc.getPage(pageNum);
  const viewport = page.getViewport({ scale: 4 });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const context = canvas.getContext("2d");
  if (!context) throw new Error("canvas 2d unavailable");
  await page.render({ canvas, canvasContext: context, viewport }).promise;
  const out = join(ticketDir, `verify-glossary-p${pageNum}.png`);
  writeFileSync(out, canvas.toBuffer("image/png"));
  console.log(`[DEBUG] wrote ${out}`);
}
