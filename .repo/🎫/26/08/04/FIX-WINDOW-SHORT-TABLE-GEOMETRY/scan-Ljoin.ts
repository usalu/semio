#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";
const png = process.argv[2];
const out = process.argv[3];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const c = createCanvas(img.width, img.height);
const ctx = c.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0,0,img.width,img.height);
const PAGE={r:247,g:243,b:227}, CANVAS={r:240,g:236,b:221};
function px(x:number,y:number){const i=(y*img.width+x)*4;return {r:data[i],g:data[i+1],b:data[i+2]};}
function dist(a:any,b:any){return Math.abs(a.r-b.r)+Math.abs(a.g-b.g)+Math.abs(a.b-b.b);}
function kind(p:any){
  const L=0.2126*p.r+0.7152*p.g+0.0722*p.b;
  if (L>80&&L<195&&Math.abs(p.r-p.g)<40&&Math.abs(p.g-p.b)<40) return "R";
  if (dist(p,PAGE)<=14) return "P";
  if (dist(p,CANVAS)<=14) return "C";
  return "o";
}
// dump every x for rows that look like joins (have horizontal rule ink in mid)
console.log(`[DEBUG] ${img.width}x${img.height}`);
for (let y=0;y<img.height;y++){
  let midR=0;
  for(let x=Math.floor(img.width*0.4);x<img.width;x++) if(kind(px(x,y))==="R") midR++;
  if (midR < img.width*0.15) continue;
  // dump full row kinds compressed
  let row="";
  for(let x=0;x<img.width;x++) row+=kind(px(x,y));
  // find border x = first sustained R from left
  let bx=-1;
  for(let x=0;x<Math.min(40,img.width);x++) if(kind(px(x,y))==="R"){bx=x;break;}
  const neigh = [-4,-3,-2,-1,0,1,2,3,4].map(dx=>{
    const x=Math.max(0,bx+dx);
    const p=px(x,y);
    return `${dx}:${kind(p)}(${p.r},${p.g},${p.b})`;
  }).join(" ");
  console.log(`[DEBUG] y=${y} bx=${bx} ${neigh}`);
}
// also write 8x nearest-neighbor zoom of full strip
const z=8;
const zoom=createCanvas(img.width*z, img.height*z);
const zctx=zoom.getContext("2d");
zctx.imageSmoothingEnabled=false;
zctx.drawImage(c,0,0,img.width,img.height,0,0,img.width*z,img.height*z);
writeFileSync(out, zoom.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${out}`);
