import { readFileSync, readdirSync, statSync, existsSync } from "fs";
import { join, relative } from "path";
const fem="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem";
function walk(dir, acc=[]) {
  for (const n of readdirSync(dir)) {
    const p=join(dir,n); const st=statSync(p);
    if (st.isDirectory()) { if (n!=="target"&&n!=="⚡️implementations"&&!n.includes("packages")) walk(p,acc); }
    else acc.push(p);
  }
  return acc;
}
const files=walk(fem);
const flats=files.filter(p=>p.endsWith(".rs")&&!p.endsWith("component.rs")&&!p.includes("/📦️packages/"));
function folderFor(flat) {
  const bytes=readFileSync(flat);
  const parent=join(flat,"..");
  let best=null;
  for (const kid of readdirSync(parent)) {
    const cand=join(parent,kid,"🦀️component.rs");
    if (!existsSync(cand)) continue;
    const b=readFileSync(cand);
    const d=Math.abs(b.length-bytes.length);
    if (!best||d<best.d) best={cand,d,a:bytes.toString("utf8"),b:b.toString("utf8")};
  }
  return best;
}
for (const f of flats) {
  if (f.endsWith("📦️lib.rs")) continue;
  const r=folderFor(f);
  if (!r||r.d===0) continue;
  // unified-ish: find first differing line
  const al=r.a.split("\n"), bl=r.b.split("\n");
  console.log("\n===", relative(fem,f), "vs", relative(fem,r.cand), "delta", r.d);
  const max=Math.max(al.length,bl.length);
  let shown=0;
  for (let i=0;i<max && shown<8;i++) {
    if (al[i]!==bl[i]) { console.log(`L${i+1}-flat:`, al[i]); console.log(`L${i+1}-fold:`, bl[i]); shown++; }
  }
}
