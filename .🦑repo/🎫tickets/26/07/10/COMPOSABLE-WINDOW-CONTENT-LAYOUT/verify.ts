#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const docs = [
  { id: "forschungsbericht", path: "print/dist/forschungsbericht.pdf", pages: [1] },
  { id: "flyer", path: "print/dist/flyer.pdf", pages: [1] },
];

const scale = 3;
const lines: string[] = ["# Composable Window Content Layout Verify", ""];

for (const doc of docs) {
  const pdfPath = join(repoRoot, doc.path);
  const docHandle = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
  for (const pageNum of doc.pages) {
    const page = await docHandle.getPage(pageNum);
    const viewport = page.getViewport({ scale });
    const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
    const context = canvas.getContext("2d");
    if (!context) throw new Error("canvas 2d unavailable");
    await page.render({ canvas, canvasContext: context, viewport }).promise;
    const out = join(ticketDir, `verify-${doc.id}-p${pageNum}.png`);
    writeFileSync(out, canvas.toBuffer("image/png"));
    lines.push(`## ${doc.id} p${pageNum}`);
    lines.push(`- raster: verify-${doc.id}-p${pageNum}.png`);
    lines.push("");
    console.log(`[DEBUG] wrote ${out}`);
  }
}

writeFileSync(join(ticketDir, "verify-log.md"), lines.join("\n"));
