#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";
const png = process.argv[2];
const out = process.argv[3];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const src = createCanvas(img.width, img.height);
src.getContext("2d").drawImage(img, 0, 0);
// Find baseline y via mid-gap
const data = src.getContext("2d").getImageData(0,0,img.width,img.height).data;
function L(x:number,y:number){const i=(y*img.width+x)*4;return 0.2126*data[i]+0.7152*data[i+1]+0.0722*data[i+2];}
const x0=Math.floor(img.width*0.42), x1=Math.floor(img.width*0.58);
let baseY=-1;
for (let y=0;y<img.height;y++){
  let r=0; for(let x=x0;x<x1;x++){const l=L(x,y); if(l>90&&l<200)r++;}
  if(r>(x1-x0)*0.45){baseY=y; break;}
}
console.log(`[DEBUG] baseY=${baseY}`);
// Extract 40px band centered on baseline, full width, scale up 4x nearest
const y0=Math.max(0,baseY-16), y1=Math.min(img.height,baseY+40);
const band = createCanvas(img.width, y1-y0);
band.getContext("2d").drawImage(src, 0,y0,img.width,y1-y0,0,0,img.width,y1-y0);
const zoom = createCanvas(img.width*2, (y1-y0)*2);
const z=zoom.getContext("2d");
z.imageSmoothingEnabled=false;
z.drawImage(band,0,0,img.width,y1-y0,0,0,img.width*2,(y1-y0)*2);
// draw markers
z.strokeStyle="red"; z.lineWidth=1;
z.beginPath(); z.moveTo(0,(baseY-y0)*2); z.lineTo(img.width*2,(baseY-y0)*2); z.stroke();
writeFileSync(out, zoom.toBuffer("image/png"));
console.log(`[DEBUG] wrote ${out} band ${y0}-${y1}`);
// classify each y in band at photo x and gap x
const PAGE=(r:number,g:number,b:number)=>Math.abs(r-247)+Math.abs(g-243)+Math.abs(b-227)<=14;
const CANVAS=(r:number,g:number,b:number)=>Math.abs(r-240)+Math.abs(g-236)+Math.abs(b-221)<=14;
for (const x of [Math.floor(img.width*0.15), Math.floor(img.width*0.45), Math.floor(img.width*0.85)]) {
  console.log(`[DEBUG] x=${x}`);
  for (let y=y0; y<y1; y++) {
    const i=(y*img.width+x)*4;
    const r=data[i],g=data[i+1],b=data[i+2];
    const l=0.2126*r+0.7152*g+0.0722*b;
    const k = (l>90&&l<200&&Math.abs(r-g)<35)?"RULE": PAGE(r,g,b)?"PAGE": CANVAS(r,g,b)?"CANVAS": (b>r+12&&b>80)?"SKY": (r>180&&g<150)?"PHOTO":`o`;
    console.log(`[DEBUG]   y=${y} dY=${y-baseY} ${k} rgb=${r},${g},${b}`);
  }
}
