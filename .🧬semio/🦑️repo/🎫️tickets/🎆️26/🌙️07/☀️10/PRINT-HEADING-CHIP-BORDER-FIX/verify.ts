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
  { id: "probe-heading", path: ".repo/🎫️/26/07/10/PRINT-HEADING-CHIP-BORDER-FIX/probe-heading.pdf", pages: [2] },
  { id: "report", path: "print/dist/report.pdf", pages: [2, 3] },
  { id: "forschungsbericht", path: "print/dist/forschungsbericht.pdf", pages: [2, 3] },
  { id: "zwischenbericht", path: "print/dist/zwischenbericht.pdf", pages: [2] },
];

const scale = 3;
const lines: string[] = ["# Print Heading Chip Border Fix Verify", ""];

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
    const out = join(ticketDir, `${doc.id}-p${pageNum}.png`);
    writeFileSync(out, canvas.toBuffer("image/png"));
    lines.push(`## ${doc.id} p${pageNum}`);
    lines.push(`- raster: ${doc.id}-p${pageNum}.png`);
    lines.push("");
    console.log(`[DEBUG] rasterized ${doc.id} p${pageNum}`);
  }
}

writeFileSync(join(ticketDir, "verify-log.md"), lines.join("\n"));
