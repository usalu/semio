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
const PAGE = {r:247,g:243,b:227};
const CANVAS = {r:240,g:236,b:221};
function px(x:number,y:number){const i=(y*img.width+x)*4;return {r:data[i],g:data[i+1],b:data[i+2]};}
function dist(a:any,b:any){return Math.abs(a.r-b.r)+Math.abs(a.g-b.g)+Math.abs(a.b-b.b);}
function kind(p:any){
  const L=0.2126*p.r+0.7152*p.g+0.0722*p.b;
  if (p.b>p.r+12 && p.b>80) return "SKY";
  if (L<55) return "DARK";
  if (L>80 && L<195 && Math.abs(p.r-p.g)<35 && Math.abs(p.g-p.b)<35) return "RULE";
  if (dist(p,PAGE)<=12) return "PAGE";
  if (dist(p,CANVAS)<=12) return "CANVAS";
  if (p.r>180 && p.g<140) return "PHOTO";
  return `other(${p.r},${p.g},${p.b})`;
}
// tighter crops around Kopfbau: find first RULE after y=180 in mid gap
const xGap = Math.floor(img.width*0.45);
const xPhoto = Math.floor(img.width*0.18);
const xL = 6;
console.log(`[DEBUG] sizes ${img.width}x${img.height}`);
for (const [name,x] of [["gap",xGap],["photo",xPhoto],["L",xL]] as const) {
  console.log(`[DEBUG] ---- ${name} x=${x} ----`);
  let prev="";
  for (let y=180; y<360; y++) {
    const k = kind(px(x,y));
    if (k!==prev || y%5===0) {
      console.log(`[DEBUG] y=${y} (${(y/scale).toFixed(2)}pt) ${k}`);
      prev=k;
    }
  }
}
// measure: after first full RULE band near y~210, what follows in gap col
let y=200;
while (y<img.height && kind(px(xGap,y))!=="RULE") y++;
const r0=y;
while (y<img.height && kind(px(xGap,y))==="RULE") y++;
const r1=y-1;
let page=0,canvasN=0;
const after=y;
while (y<img.height && (kind(px(xGap,y))==="PAGE"||kind(px(xGap,y))==="CANVAS")) {
  if (kind(px(xGap,y))==="PAGE") page++; else canvasN++;
  y++;
}
console.log(`[DEBUG] baseline RULE ${r0}-${r1} (${((r1-r0+1)/scale).toFixed(2)}pt)`);
console.log(`[DEBUG] after baseline: PAGE=${(page/scale).toFixed(2)}pt CANVAS=${(canvasN/scale).toFixed(2)}pt next=${kind(px(xGap,y))} at y=${y}`);
// photo: first RULE then pad then PHOTO/SKY
y=200;
while (y<img.height && kind(px(xPhoto,y))!=="RULE") y++;
const pr0=y;
while (y<img.height && kind(px(xPhoto,y))==="RULE") y++;
const pr1=y-1;
let padPage=0,padCanvas=0;
while (y<img.height) {
  const k=kind(px(xPhoto,y));
  if (k==="PAGE") padPage++;
  else if (k==="CANVAS") padCanvas++;
  else break;
  y++;
}
console.log(`[DEBUG] photo-col first RULE ${pr0}-${pr1}; pad PAGE=${(padPage/scale).toFixed(2)} CANVAS=${(padCanvas/scale).toFixed(2)} next=${kind(px(xPhoto,y))} y=${y}`);
