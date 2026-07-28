import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync("dist/zwischenbericht.pdf")), useSystemFonts: true }).promise;
const scale=8; const pg=Number(process.argv[2]), y=Number(process.argv[3]), x0=Number(process.argv[4]), x1=Number(process.argv[5]);
const page=await doc.getPage(pg); const vp=page.getViewport({scale}); const c=createCanvas(Math.ceil(vp.width),Math.ceil(vp.height));
await page.render({canvas:c,canvasContext:c.getContext("2d"),viewport:vp}).promise; const d=c.getContext("2d").getImageData(0,0,c.width,c.height); const W=c.width;
// at row y, find runs where pixel is "dark-ish" (border) vs light (gap). Report light gaps inside [x0,x1].
let gaps=[],cur=null;
for(let x=x0;x<=x1;x++){const i=(y*W+x)*4; const lum=(d.data[i]+d.data[i+1]+d.data[i+2])/3; const isBorder=lum<200;
  if(!isBorder){ if(cur)cur.x1=x; else cur={x0:x,x1:x}; } else { if(cur){gaps.push(cur);cur=null;} } }
if(cur)gaps.push(cur);
const big=gaps.filter(g=>g.x1-g.x0>=3);
console.log(`P${pg} y=${(y/8).toFixed(1)}pt border scan x[${(x0/8).toFixed(0)}-${(x1/8).toFixed(0)}pt]: ${big.length? big.length+" gaps": "CONTINUOUS (no gaps>3px)"}`);
for(const g of big.slice(0,12)) console.log(`   gap ${(g.x0/8).toFixed(1)}-${(g.x1/8).toFixed(1)}pt (${g.x1-g.x0}px)`);
