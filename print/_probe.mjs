import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const [, , pdf, pg, sc] = process.argv;
const scale = Number(sc||8);
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdf)), useSystemFonts: true }).promise;
const page = await doc.getPage(Number(pg)); const vp = page.getViewport({ scale });
const c = createCanvas(Math.ceil(vp.width), Math.ceil(vp.height));
const ctx = c.getContext("2d");
await page.render({ canvas:c, canvasContext:ctx, viewport:vp }).promise;
const data = ctx.getImageData(0,0,c.width,c.height).data;
const at = (x,y)=>{const i=(y*c.width+x)*4;return [data[i],data[i+1],data[i+2]];};
const isDark = ([r,g,b])=> r<170 && g<170 && b<170;
const isPage = ([r,g])=> r>=245 && g>=241;
const isCanvas = ([r,g,b])=> Math.abs(r-240)<4 && Math.abs(g-236)<4 && Math.abs(b-221)<4;
const L=748,R=4194;
// scan just-inside-left and just-inside-right fill for page-bg or seam, across many rows
let leftBad=0,rightBad=0,tot=0;
for(let y=1720;y<3050;y+=3){
  for(let dx=3;dx<=10;dx++){ tot++; if(isPage(at(L+dx,y)))leftBad++; if(isPage(at(R-dx,y)))rightBad++; }
}
console.log("edge page-bg pixels  left",leftBad,"right",rightBad,"of",tot);
// corners: page-bg just inside both borders below/above rules
const corner=(x0,x1,y0,y1)=>{let n=0,t=0;for(let y=y0;y<y1;y++)for(let x=x0;x<x1;x++){t++;if(isPage(at(x,y)))n++;}return `${n}/${t}`;};
// top rule y ~1650, bottom of card ~ find last dark long rule
console.log("TL corner", corner(L+2,L+34,1656,1686));
console.log("TR corner", corner(R-34,R-2,1656,1686));
