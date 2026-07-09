#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const distDir = join(repoRoot, "print/dist");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const jobs: readonly (readonly [string, number, string])[] = [
	["report.pdf", 4, "report-p4-window"],
	["report-dark.pdf", 4, "report-dark-p4-window"],
];

for (const [name, pageNum, outStem] of jobs) {
	const pdfPath = join(distDir, name);
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
