import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { readFileSync, writeFileSync } from "node:fs";

const pdfjsEntry = fileURLToPath(new URL("pdfjs-dist/legacy/build/pdf.mjs", import.meta.resolve("pdfjs-dist")));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");

const path = process.argv[2];
const page = Number(process.argv[3] ?? 1);
const out = process.argv[4];

const data = readFileSync(path);
const doc = await pdfjs.getDocument({ data: new Uint8Array(data), useSystemFonts: true }).promise;
const p = await doc.getPage(page);
const viewport = p.getViewport({ scale: 3 });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await p.render({ canvas, canvasContext: ctx, viewport }).promise;
writeFileSync(out, canvas.toBuffer("image/png"));
console.log("wrote", out);
