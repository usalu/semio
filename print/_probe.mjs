import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const { createCanvas } = createRequire(import.meta.url)("@napi-rs/canvas");
const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
const [,, pdfPath, pageArg, needle] = process.argv;
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const S = 8; // px per pt
const pg = Number(pageArg);
const page = await doc.getPage(pg);
const vp = page.getViewport({ scale: S });
const W = Math.ceil(vp.width), H = Math.ceil(vp.height);
const c = createCanvas(W, H);
await page.render({ canvas: c, canvasContext: c.getContext("2d"), viewport: vp }).promise;
const d = c.getContext("2d").getImageData(0, 0, W, H).data;
// locate needle text center (pdfjs coords -> device via viewport transform)
const tc = await page.getTextContent();
let hit = null;
for (const it of tc.items) {
  if (it.str && it.str.includes(needle)) {
    const [a,b,cc,dd,e,f] = it.transform;
    // baseline start in pdf space; convert corners to device
    const p1 = vp.convertToViewportPoint(e, f);
    const p2 = vp.convertToViewportPoint(e + it.width, f + it.height);
    hit = { str: it.str, x0: Math.min(p1[0],p2[0]), x1: Math.max(p1[0],p2[0]), y0: Math.min(p1[1],p2[1]), y1: Math.max(p1[1],p2[1]) };
    break;
  }
}
if (!hit) { console.log(`needle "${needle}" not found on p${pg}`); process.exit(1); }
const cx = Math.round((hit.x0 + hit.x1)/2), cy = Math.round((hit.y0 + hit.y1)/2);
console.log(`FOUND "${hit.str.slice(0,40)}" textbox px x[${hit.x0.toFixed(0)}-${hit.x1.toFixed(0)}] y[${hit.y0.toFixed(0)}-${hit.y1.toFixed(0)}]  center=(${cx},${cy})`);
console.log(`  => text pt x[${(hit.x0/S).toFixed(2)}-${(hit.x1/S).toFixed(2)}] y[${(hit.y0/S).toFixed(2)}-${(hit.y1/S).toFixed(2)}]`);
function classify(r,g,b){
  const lum=(r+g+b)/3;
  if (lum>170) return "TEXT";        // bright light text
  if (lum>90)  return "RULE";        // gray ~#7b827d (123,130,125) lum~126
  if (lum>28)  return "CANVAS";      // cell fill ~#0c1c21 / panels
  return "BG";                        // dark base
}
function runs(vals){ // vals: array of {p, cls}
  const out=[]; let cur=null;
  for(const v of vals){ if(cur&&cur.cls===v.cls){cur.p1=v.p;} else {if(cur)out.push(cur); cur={cls:v.cls,p0:v.p,p1:v.p};}}
  if(cur)out.push(cur); return out;
}
function px(v){return (v/S).toFixed(2);}
// horizontal scan at cy
let hv=[]; for(let x=0;x<W;x++){const i=(cy*W+x)*4; hv.push({p:x,cls:classify(d[i],d[i+1],d[i+2])});}
// vertical scan at cx
let vv=[]; for(let y=0;y<H;y++){const i=(y*W+cx)*4; vv.push({p:y,cls:classify(d[i],d[i+1],d[i+2])});}
console.log(`\n--- HORIZONTAL scan @ y=${cy} (pt ${px(cy)}), runs >=1px, near cell ---`);
for(const r of runs(hv)){ if(r.p1-r.p0>=1 && r.cls!=="BG" || (r.cls==="RULE")){ const w=(r.p1-r.p0+1); if(w>=2||r.cls==="RULE") console.log(`  x ${px(r.p0)}-${px(r.p1)}pt  ${r.cls} (${(w/S).toFixed(2)}pt)`);}}
console.log(`\n--- VERTICAL scan @ x=${cx} (pt ${px(cx)}) ---`);
for(const r of runs(vv)){ const w=(r.p1-r.p0+1); if(w>=2||r.cls==="RULE") console.log(`  y ${px(r.p0)}-${px(r.p1)}pt  ${r.cls} (${(w/S).toFixed(2)}pt)`);}
