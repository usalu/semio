import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(process.argv[2])), useSystemFonts: true }).promise;
const scale = 8; const cache={};
async function img(pg){ if(cache[pg])return cache[pg]; const page=await doc.getPage(pg); const vp=page.getViewport({scale}); const c=createCanvas(Math.ceil(vp.width),Math.ceil(vp.height)); await page.render({canvas:c,canvasContext:c.getContext("2d"),viewport:vp}).promise; return cache[pg]={d:c.getContext("2d").getImageData(0,0,c.width,c.height),W:c.width};}
function hex(n){return n.toString(16).padStart(2,'0');}
async function sample(pg,x,y){ const {d,W}=await img(pg); let r=0,g=0,b=0,n=0; for(let dy=-3;dy<=3;dy++)for(let dx=-3;dx<=3;dx++){const i=((y+dy)*W+(x+dx))*4;r+=d.data[i];g+=d.data[i+1];b+=d.data[i+2];n++;} r=Math.round(r/n);g=Math.round(g/n);b=Math.round(b/n); return `#${hex(r)}${hex(g)}${hex(b)}`; }
const pts = JSON.parse(process.argv[3]);
for (const p of pts) console.log(`${p[0].padEnd(34)} p${p[1]} (${p[2]},${p[3]}): ${await sample(p[1],p[2],p[3])}`);
