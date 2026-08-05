#!/usr/bin/env bun
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const [pdfPath, pageArg, outPng, yTopArg, yBotArg, x0Arg, x1Arg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(Number(pageArg));
const scale = 4;
const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
const pageHpt = viewport.height / scale;
const yTop = Number(yTopArg), yBot = Number(yBotArg);
const x0 = Math.floor(Number(x0Arg ?? 60) * scale);
const x1 = Math.ceil(Number(x1Arg ?? 540) * scale);
const cy0 = Math.floor((pageHpt - yTop) * scale);
const cy1 = Math.ceil((pageHpt - yBot) * scale);
const y0 = Math.min(cy0, cy1), y1 = Math.max(cy0, cy1);
// Find baseline in this band, then extract ±16px above / +40px below, 3x zoom no marker
const bandH = y1-y0;
const tmp = createCanvas(x1-x0, bandH);
tmp.getContext("2d").drawImage(canvas, x0, y0, x1-x0, bandH, 0, 0, x1-x0, bandH);
const { data } = tmp.getContext("2d").getImageData(0,0,x1-x0,bandH);
const mid0 = Math.floor((x1-x0)*0.42), mid1 = Math.floor((x1-x0)*0.58);
let base=-1;
for(let y=0;y<bandH;y++){
  let r=0; for(let x=mid0;x<mid1;x++){const i=(y*(x1-x0)+x)*4; const L=0.2126*data[i]+0.7152*data[i+1]+0.0722*data[i+2]; if(L>90&&L<200)r++;}
  if(r>(mid1-mid0)*0.45){base=y;break;}
}
const sy0=Math.max(0,base-20), sy1=Math.min(bandH, base+48);
const slice=createCanvas(x1-x0, sy1-sy0);
slice.getContext("2d").drawImage(tmp, 0,sy0,x1-x0,sy1-sy0,0,0,x1-x0,sy1-sy0);
const z=3;
const zoom=createCanvas((x1-x0)*z, (sy1-sy0)*z);
const zc=zoom.getContext("2d"); zc.imageSmoothingEnabled=false;
zc.drawImage(slice,0,0,x1-x0,sy1-sy0,0,0,(x1-x0)*z,(sy1-sy0)*z);
writeFileSync(outPng, zoom.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${outPng} base=${base} slice=${sy0}-${sy1}`);
