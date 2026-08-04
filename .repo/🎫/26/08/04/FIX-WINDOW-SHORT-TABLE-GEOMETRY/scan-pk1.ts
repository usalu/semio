#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
const pngPath = process.argv[2];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const scale = 4;
function px(x:number,y:number){const i=(y*img.width+x)*4;return {r:data[i],g:data[i+1],b:data[i+2],L:0.2126*data[i]+0.7152*data[i+1]+0.0722*data[i+2]};}
// Sample photo column and chip gap column
const cols = [
  ["photo", Math.floor(img.width*0.18)],
  ["chipgap", Math.floor(img.width*0.45)],
  ["meta", Math.floor(img.width*0.72)],
  ["Lborder", 4],
];
// Find Kopfbau region: look for dark text density jump around mid page of crop
for (const [name,x] of cols) {
  console.log(`[DEBUG] === col ${name} x=${x} ===`);
  for (let y=120; y<340; y++) {
    const p = px(x,y);
    const kind = p.L < 60 ? "DARK" : (p.b>p.r+15 && p.b>80 ? "SKY" : (p.L>80&&p.L<190&&Math.abs(p.r-p.g)<30&&Math.abs(p.g-p.b)<30 ? "RULE" : (p.L>220 ? "PAGE" : (p.L>190 ? "CANVAS?" : "mid"))));
    if (y%2===0 || kind==="RULE" || kind==="SKY") console.log(`[DEBUG] y=${y} (${(y/scale).toFixed(2)}pt) RGB=${p.r},${p.g},${p.b} L=${p.L.toFixed(0)} ${kind}`);
  }
}
