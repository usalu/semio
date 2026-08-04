#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
const png = process.argv[2];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const c = createCanvas(img.width, img.height);
c.getContext("2d").drawImage(img, 0, 0);
const { data } = c.getContext("2d").getImageData(0,0,img.width,img.height);
function rgb(x:number,y:number){const i=(y*img.width+x)*4;return [data[i],data[i+1],data[i+2]] as const;}
function isPage(r:number,g:number,b:number){return Math.abs(r-247)+Math.abs(g-243)+Math.abs(b-227)<=14;}
function isCanvas(r:number,g:number,b:number){return Math.abs(r-240)+Math.abs(g-236)+Math.abs(b-221)<=14;}
function isRule(r:number,g:number,b:number){const L=0.2126*r+0.7152*g+0.0722*b;return L>90&&L<200&&Math.abs(r-g)<35&&Math.abs(g-b)<35;}
// Find baseline y
let baseY=-1;
for(let y=0;y<img.height;y++){
  let n=0; for(let x=Math.floor(img.width*0.4); x<Math.floor(img.width*0.6); x++){const [r,g,b]=rgb(x,y); if(isRule(r,g,b))n++;}
  if(n>img.width*0.08){baseY=y;break;}
}
console.log(`[DEBUG] baseY=${baseY}`);
// Scan row just above baseline for canvas vs page runs (chip detection)
const y = baseY-2;
let runs: {k:string,x0:number,x1:number}[]=[];
for(let x=0;x<img.width;x++){
  const [r,g,b]=rgb(x,y);
  const k = isRule(r,g,b)?"RULE": isCanvas(r,g,b)?"CANVAS": isPage(r,g,b)?"PAGE":"OTHER";
  const last=runs[runs.length-1];
  if(last && last.k===k && last.x1===x-1) last.x1=x; else runs.push({k,x0:x,x1:x});
}
for(const r of runs){ if((r.x1-r.x0)>8) console.log(`[DEBUG] y=${y} ${r.k} x=${r.x0}-${r.x1} w=${((r.x1-r.x0+1)/4).toFixed(1)}pt`); }
// Border continuity: find border x, measure luminance along it
let bx=-1,best=0;
for(let x=0;x<Math.min(200,img.width);x++){
  let n=0; for(let y=0;y<img.height;y++){const [r,g,b]=rgb(x,y); if(isRule(r,g,b))n++;}
  if(n>best){best=n;bx=x;}
}
console.log(`[DEBUG] borderX=${bx} ruleRows=${best}/${img.height}`);
// luminance profile at joins: find horizontal rules intersecting border
const scale=4;
for(let y=1;y<img.height-1;y++){
  // horizontal rule near mid
  let midRule=0; for(let x=Math.floor(img.width*0.3);x<Math.floor(img.width*0.5);x++){const [r,g,b]=rgb(x,y); if(isRule(r,g,b))midRule++;}
  if(midRule<(img.width*0.2)*0.4) continue;
  // sample border luminance neighborhood
  const samples=[];
  for(let dy=-6; dy<=6; dy++){
    const [r,g,b]=rgb(bx,y+dy);
    const L=0.2126*r+0.7152*g+0.0722*b;
    samples.push({dy,L:L.toFixed(0),r,g,b,k:isRule(r,g,b)?"R":isPage(r,g,b)?"P":isCanvas(r,g,b)?"C":"o"});
  }
  const hasPage = samples.some(s=>s.k==="P");
  const minL = Math.min(...samples.map(s=>+s.L));
  const maxL = Math.max(...samples.map(s=>+s.L));
  console.log(`[DEBUG] H-rule y=${y} border neighborhood page=${hasPage} Lrange=${minL}-${maxL}`);
  console.log(`[DEBUG]   `+samples.map(s=>`${s.dy}:${s.k}${s.L}`).join(" "));
}
