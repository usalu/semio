import { createCanvas } from "@napi-rs/canvas";
import * as pdfjs from "pdfjs-dist/legacy/build/pdf.mjs";

const path = process.argv[2];
const page = Number(process.argv[3] ?? 1);
const out = process.argv[4];

const data = await Bun.file(path).arrayBuffer();
const doc = await pdfjs.getDocument({ data: new Uint8Array(data) }).promise;
const p = await doc.getPage(page);
const viewport = p.getViewport({ scale: 3 });
const canvas = createCanvas(viewport.width, viewport.height);
const ctx = canvas.getContext("2d");
await p.render({ canvasContext: ctx as any, viewport }).promise;
await Bun.write(out, canvas.toBuffer("image/png"));
console.log("wrote", out);
