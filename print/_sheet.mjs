import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(process.argv[2])), useSystemFonts: true }).promise;
const N=doc.numPages, cols=6, cw=200, ch=283, rows=Math.ceil(N/cols);
const sheet=createCanvas(cols*cw, rows*ch); const sctx=sheet.getContext("2d"); sctx.fillStyle="#fff"; sctx.fillRect(0,0,sheet.width,sheet.height);
for(let i=1;i<=N;i++){const pg=await doc.getPage(i);const vp=pg.getViewport({scale:1});const s=Math.min(cw/vp.width,ch/vp.height);const v2=pg.getViewport({scale:s});
  const c=createCanvas(Math.ceil(v2.width),Math.ceil(v2.height));await pg.render({canvas:c,canvasContext:c.getContext("2d"),viewport:v2}).promise;
  const col=(i-1)%cols, row=Math.floor((i-1)/cols); sctx.drawImage(c,col*cw,row*ch); sctx.fillStyle="#f00"; sctx.font="14px sans"; sctx.fillText(String(i),col*cw+4,row*ch+16);}
writeFileSync(process.argv[3], sheet.toBuffer("image/png")); console.log("wrote",process.argv[3]);
