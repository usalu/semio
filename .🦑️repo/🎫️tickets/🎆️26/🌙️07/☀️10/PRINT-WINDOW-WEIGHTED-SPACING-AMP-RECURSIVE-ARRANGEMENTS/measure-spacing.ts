#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildPrintDocument } from "../../../../../../print/📜️script.ts";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../");
const texAbs = join(ticketDir, "verify-cover.tex");
const outDir = join(ticketDir, "dist");
const pdfPath = join(outDir, "verify-cover.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

await buildPrintDocument(texAbs, outDir);

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(1);
const scale = 3;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
writeFileSync(join(ticketDir, "cover-spacing.png"), canvas.toBuffer("image/png"));

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
const bands: { start: number; end: number }[] = [];
let inBand = false;
let start = 0;
for (let y = 0; y < height; y++) {
  const active = rowInk[y]! > width * 0.01;
  if (active && !inBand) {
    inBand = true;
    start = y;
  }
  if (!active && inBand) {
    bands.push({ start, end: y - 1 });
    inBand = false;
  }
}
if (inBand) bands.push({ start, end: height - 1 });
const verticalGaps: number[] = [];
for (let i = 0; i < bands.length - 1; i++) verticalGaps.push(bands[i + 1]!.start - bands[i]!.end);

const colInk = new Uint32Array(width);
for (let x = 0; x < width; x++) {
  let count = 0;
  for (let y = 0; y < height; y++) {
    const i = (y * width + x) * 4;
    const dr = Math.abs(data[i]! - bg[0]!);
    const dg = Math.abs(data[i + 1]! - bg[1]!);
    const db = Math.abs(data[i + 2]! - bg[2]!);
    if (dr + dg + db > inkThreshold) count++;
  }
  colInk[x] = count;
}
const colBands: { start: number; end: number }[] = [];
inBand = false;
for (let x = 0; x < width; x++) {
  const active = colInk[x]! > height * 0.01;
  if (active && !inBand) {
    inBand = true;
    start = x;
  }
  if (!active && inBand) {
    colBands.push({ start, end: x - 1 });
    inBand = false;
  }
}
if (inBand) colBands.push({ start, end: width - 1 });
const horizontalGaps: number[] = [];
for (let i = 0; i < colBands.length - 1; i++) horizontalGaps.push(colBands[i + 1]!.start - colBands[i]!.end);

const pxPerPt = scale * (96 / 72);
const expectedPx = 4 * pxPerPt;
const tol = 1.5;
const summarize = (gaps: number[], label: string) =>
  gaps.map((g, i) => {
    const pt = +(g / pxPerPt).toFixed(2);
    const ok = Math.abs(pt - 4) < tol;
    return { label, gap: i + 1, px: g, pt, ok };
  });
const vertical = summarize(verticalGaps.slice(0, 12), "vertical");
const horizontal = summarize(horizontalGaps.slice(0, 12), "horizontal");
console.log("[DEBUG] expected gap px", +expectedPx.toFixed(2));
console.log("[DEBUG] vertical gaps", vertical);
console.log("[DEBUG] horizontal gaps", horizontal);
const log = [
  "# Print Window Weighted Spacing Verify",
  "",
  "Cover raster: cover-spacing.png",
  `Expected gap: 4pt (~${expectedPx.toFixed(1)}px at scale ${scale})`,
  "",
  "## Vertical gaps",
  "| Gap | px | pt | ok |",
  "| --- | --- | --- | --- |",
  ...vertical.map((s) => `| ${s.gap} | ${s.px} | ${s.pt} | ${s.ok ? "yes" : "no"} |`),
  "",
  "## Horizontal gaps",
  "| Gap | px | pt | ok |",
  "| --- | --- | --- | --- |",
  ...horizontal.map((s) => `| ${s.gap} | ${s.px} | ${s.pt} | ${s.ok ? "yes" : "no"} |`),
].join("\n");
writeFileSync(join(ticketDir, "verify-log.md"), log);
