#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const pdfPath = join(repoRoot, "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const pageNum = Number(process.env.PAGE ?? 5);
const page = await doc.getPage(pageNum);
const scale = 3;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
writeFileSync(join(ticketDir, `report-p${pageNum}-spacing.png`), canvas.toBuffer("image/png"));

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
const gaps: number[] = [];
for (let i = 0; i < bands.length - 1; i++) gaps.push(bands[i + 1]!.start - bands[i]!.end);
const pxPerEm = (12 * scale * 96) / 72;
const expectedPx = 0.2 * pxPerEm;
const summary = gaps.map((g, i) => {
	const em = +(g / pxPerEm).toFixed(3);
	const ok = Math.abs(em - 0.2) < 0.08;
	return { gap: i + 1, px: g, em, ok };
});
console.log("[DEBUG] spacing bands", bands.length, "gaps", summary);
console.log("[DEBUG] expected single unit px", +expectedPx.toFixed(2));
const log = [
	"# Print Uniform Block Spacing Verify",
	"",
	`Page ${pageNum} raster: report-p${pageNum}-spacing.png`,
	`Expected gap: 0.2em (~${expectedPx.toFixed(1)}px at scale ${scale})`,
	"",
	"| Gap | px | em | ok |",
	"| --- | --- | --- | --- |",
	...summary.map((s) => `| ${s.gap} | ${s.px} | ${s.em} | ${s.ok ? "yes" : "no"} |`),
].join("\n");
writeFileSync(join(ticketDir, "verify-log.md"), log);
