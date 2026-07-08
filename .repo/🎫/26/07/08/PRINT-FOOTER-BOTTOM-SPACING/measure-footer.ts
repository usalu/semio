#!/usr/bin/env bun
import { readFileSync } from "node:fs";
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
const page = await doc.getPage(2);
const viewport = page.getViewport({ scale: 2 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const context = canvas.getContext("2d");
if (!context) throw new Error("canvas 2d unavailable");
await page.render({ canvas, canvasContext: context, viewport }).promise;
const { data, width, height } = context.getImageData(0, 0, canvas.width, canvas.height);
const chromeBase = { r: 247, g: 243, b: 227 };
const canvasBg = { r: 240, g: 236, b: 221 };
const scanStart = Math.floor(height * 0.75);
let footerTop = -1;
let footerBottom = -1;
for (let y = height - 1; y >= scanStart; y--) {
	let chromePixels = 0;
	for (let x = 0; x < width; x++) {
		const i = (y * width + x) * 4;
		const dr = Math.abs(data[i] - chromeBase.r);
		const dg = Math.abs(data[i + 1] - chromeBase.g);
		const db = Math.abs(data[i + 2] - chromeBase.b);
		if (dr + dg + db < 30) chromePixels++;
	}
	if (chromePixels > width * 0.5) {
		footerBottom = y;
		break;
	}
}
for (let y = scanStart; y < height; y++) {
	let chromePixels = 0;
	for (let x = 0; x < width; x++) {
		const i = (y * width + x) * 4;
		const dr = Math.abs(data[i] - chromeBase.r);
		const dg = Math.abs(data[i + 1] - chromeBase.g);
		const db = Math.abs(data[i + 2] - chromeBase.b);
		if (dr + dg + db < 30) chromePixels++;
	}
	if (chromePixels > width * 0.5) {
		footerTop = y;
		break;
	}
}
let contentBottom = -1;
for (let y = footerTop - 1; y >= 0; y--) {
	let contentPixels = 0;
	for (let x = Math.floor(width * 0.1); x < Math.floor(width * 0.9); x++) {
		const i = (y * width + x) * 4;
		const dr = Math.abs(data[i] - canvasBg.r);
		const dg = Math.abs(data[i + 1] - canvasBg.g);
		const db = Math.abs(data[i + 2] - canvasBg.b);
		if (dr + dg + db > 25) contentPixels++;
	}
	if (contentPixels > width * 0.05) {
		contentBottom = y;
		break;
	}
}
const gapContentFooter = footerTop - contentBottom - 1;
const gapPx = height - 1 - footerBottom;
const footerHeightPx = footerBottom - footerTop;
console.log(`[DEBUG] page height px=${height} footer top=${footerTop} bottom=${footerBottom} height=${footerBottom - footerTop} gap below footer=${height - 1 - footerBottom}px content bottom=${contentBottom} gap content→footer=${gapContentFooter}px (${(gapContentFooter / (height / 842)).toFixed(1)}pt est)`);
