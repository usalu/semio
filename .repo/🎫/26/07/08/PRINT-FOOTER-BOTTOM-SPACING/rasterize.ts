#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../../");
const pdfPath = join(repoRoot, "print/dist/paper.pdf");
const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
for (let pageNum = 1; pageNum <= doc.numPages; pageNum++) {
	const page = await doc.getPage(pageNum);
	const viewport = page.getViewport({ scale: 2 });
	const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
	const context = canvas.getContext("2d");
	if (!context) throw new Error("canvas 2d unavailable");
	await page.render({ canvas, canvasContext: context, viewport }).promise;
	writeFileSync(join(ticketDir, `report-p${pageNum}-before.png`), canvas.toBuffer("image/png"));
	const cropH = Math.ceil(viewport.height * 0.2);
	const crop = createCanvas(Math.ceil(viewport.width), cropH);
	const cropCtx = crop.getContext("2d");
	if (!cropCtx) throw new Error("crop ctx unavailable");
	cropCtx.drawImage(canvas, 0, viewport.height - cropH, viewport.width, cropH, 0, 0, viewport.width, cropH);
	writeFileSync(join(ticketDir, `report-p${pageNum}-footer-crop.png`), crop.toBuffer("image/png"));
	console.log(`[DEBUG] wrote report-p${pageNum}-before.png and footer crop`);
}
