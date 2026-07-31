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
  { id: "report", path: "print/dist/report.pdf", pages: [4, 5] },
  { id: "paper", path: "print/dist/paper.pdf", pages: [1, 2] },
  { id: "flyer", path: "print/dist/flyer.pdf", pages: [1] },
  { id: "zwischenbericht", path: "print/dist/zwischenbericht.pdf", pages: [1, 5] },
];

const scale = 3;
const lines: string[] = ["# Print Footer Visibility Verify", ""];

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

    const { data, width, height } = context.getImageData(0, 0, canvas.width, canvas.height);
    const bg = [247, 243, 227];
    const inkThreshold = 90;
    const rowInk = new Uint32Array(height);
    for (let y = 0; y < height; y++) {
      let count = 0;
      for (let x = 0; x < width; x++) {
        const i = (y * width + x) * 4;
        const dr = Math.abs(data[i]! - bg[0]!);
        const dg = Math.abs(data[i + 1]! - bg[1]!);
        const db = Math.abs(data[i + 2]! - bg[2]!);
        if (dr + dg + db > inkThreshold) count++;
      }
      rowInk[y] = count;
    }
    const topInk = rowInk.findIndex((n) => n > width * 0.01);
    const bottomInk = height - 1 - [...rowInk].reverse().findIndex((n) => n > width * 0.01);
    const topGapPx = topInk > 0 ? topInk : 0;
    const bottomGapPx = bottomInk < height - 1 ? height - 1 - bottomInk : 0;
    lines.push(`## ${doc.id} p${pageNum}`);
    lines.push(`- raster: ${doc.id}-p${pageNum}.png`);
    lines.push(`- top gap px: ${topGapPx}`);
    lines.push(`- bottom gap px: ${bottomGapPx}`);
    lines.push(`- gap delta px: ${Math.abs(topGapPx - bottomGapPx)}`);
    lines.push("");
    console.log(`[DEBUG] ${doc.id} p${pageNum} top=${topGapPx} bottom=${bottomGapPx}`);
  }
}

writeFileSync(join(ticketDir, "verify-log.md"), lines.join("\n"));
