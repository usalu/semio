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
const rightBorderAt = (y)=>{ for(let x=c.width-1;x>3000;x--){ if(isDark(at(x,y))) return x; } return null; };
const leftBorderAt = (y)=>{ for(let x=600;x<1400;x++){ if(isDark(at(x,y))) return x; } return null; };
const hExtent=(y)=>{let xs=null,xe=null;for(let x=700;x<4300;x++){if(isDark(at(x,y))||isDark(at(x,y-1))||isDark(at(x,y+1))){if(xs===null)xs=x;xe=x;}}return[xs,xe];};
const findRule=(y0,y1)=>{for(let y=y0;y<y1;y++){let n=0;for(let x=800;x<4000;x+=50)if(isDark(at(x,y)))n++;if(n>50)return y;}return null;};
const topRuleY=findRule(1600,1780);
const divY=findRule(2150,2400);
console.log("topRuleY",topRuleY,"extent",topRuleY&&hExtent(topRuleY));
console.log("dividerY",divY,"extent",divY&&hExtent(divY));
for (const [lbl,y] of [["band",1900],["header",2400],["comp",2750]]) console.log(lbl.padEnd(7),"L=",leftBorderAt(y),"R=",rightBorderAt(y));
// TR corner intrusion: rectangle just inside right border, just below top rule
if(topRuleY){const rb=rightBorderAt(1900);let intr=0,tot=0;for(let y=topRuleY+2;y<topRuleY+30;y++)for(let x=rb-30;x<rb;x++){tot++;if(isPage(at(x,y)))intr++;}console.log("TRcorner page-bg intrusion",intr,"/",tot);}
