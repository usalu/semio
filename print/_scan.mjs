import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(process.argv[2])), useSystemFonts: true }).promise;
const scale = 8; const cache={};
async function img(pg){ if(cache[pg])return cache[pg]; const page=await doc.getPage(pg); const vp=page.getViewport({scale}); const c=createCanvas(Math.ceil(vp.width),Math.ceil(vp.height)); await page.render({canvas:c,canvasContext:c.getContext("2d"),viewport:vp}).promise; return cache[pg]={d:c.getContext("2d").getImageData(0,0,c.width,c.height),W:c.width};}
function hex(n){return n.toString(16).padStart(2,'0');}
// vertical scan at column x: report color runs (grouping similar colors)
async function vscan(pg,x,y0,y1){ const {d,W}=await img(pg); let runs=[],cur=null;
  for(let y=y0;y<=y1;y++){const i=(y*W+x)*4;const c=`${hex(d.data[i])}${hex(d.data[i+1])}${hex(d.data[i+2])}`;
    if(cur&&cur.c===c){cur.y1=y;}else{if(cur)runs.push(cur);cur={c,y0:y,y1:y};}}
  if(cur)runs.push(cur);
  // merge tiny runs, print runs >= 3px
  for(const r of runs){ if(r.y1-r.y0>=2) console.log(`  y${(r.y0/8).toFixed(1)}-${(r.y1/8).toFixed(1)}pt #${r.c}`);} }
console.log(`=== P${process.argv[3]} col x=${process.argv[4]} (${(process.argv[4]/8).toFixed(0)}pt) ===`);
await vscan(Number(process.argv[3]), Number(process.argv[4]), Number(process.argv[5]||600), Number(process.argv[6]||1400));
