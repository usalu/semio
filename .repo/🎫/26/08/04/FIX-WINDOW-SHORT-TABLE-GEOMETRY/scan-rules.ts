#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
const png = process.argv[2];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const c = createCanvas(img.width, img.height);
const ctx = c.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const scale = 4;
const PAGE={r:247,g:243,b:227}, CANVAS={r:240,g:236,b:221};
function px(x:number,y:number){const i=(y*img.width+x)*4;return {r:data[i],g:data[i+1],b:data[i+2]};}
function dist(a:any,b:any){return Math.abs(a.r-b.r)+Math.abs(a.g-b.g)+Math.abs(a.b-b.b);}
function kind(p:any){
  const L=0.2126*p.r+0.7152*p.g+0.0722*p.b;
  if (p.b>p.r+12 && p.b>80) return "SKY";
  if (L<55) return "DARK";
  if (L>80 && L<195 && Math.abs(p.r-p.g)<35 && Math.abs(p.g-p.b)<35) return "RULE";
  if (dist(p,PAGE)<=14) return "PAGE";
  if (dist(p,CANVAS)<=14) return "CANVAS";
  if (p.r>180 && p.g<150) return "PHOTO";
  return `o(${p.r},${p.g},${p.b})`;
}
// For seam-pk1 and pad-pk1: dump every row at several x
for (const x of [80, 200, 400, 900, 1600, 1900]) {
  if (x>=img.width) continue;
  console.log(`\n[DEBUG] x=${x}`);
  // find RULE bands
  let y=0;
  while (y<img.height) {
    while (y<img.height && kind(px(x,y))!=="RULE") y++;
    if (y>=img.height) break;
    const y0=y;
    while (y<img.height && kind(px(x,y))==="RULE") y++;
    const y1=y-1;
    const before = y0>0?kind(px(x,y0-1)):"(start)";
    const after = y<img.height?kind(px(x,y)):"(end)";
    // measure after strip
    let p=0,cv=0,o=0; let yy=y; const yStart=y;
    while (yy<img.height && yy<yStart+40) {
      const k=kind(px(x,yy));
      if (k==="PAGE") p++; else if (k==="CANVAS") cv++; else if (k==="RULE") break; else { o++; if (k==="PHOTO"||k==="SKY"||k.startsWith("o")) break; }
      yy++;
    }
    console.log(`[DEBUG] RULE ${y0}-${y1} (${((y1-y0+1)/scale).toFixed(2)}pt) before=${before} after=${after} then PAGE=${(p/scale).toFixed(2)} CANVAS=${(cv/scale).toFixed(2)} next=${yy<img.height?kind(px(x,yy)):"end"}@${yy}`);
  }
}
