import {readdirSync,readFileSync,statSync} from "node:fs";
import {createHash} from "node:crypto";
const W="/Users/ueli/Documents/semio",L=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const m=await import(L+"/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs");
const root=W+"/.cache/b/g/m";let ok=0,bad=0,badSha=0,errs=new Map();const t0=performance.now();
for(const [dir,fn] of [["build-input-closures",m.captureSourceInputInventoryV1],["build-lock-closures",m.captureCargoLockInventoryV1]]){
 for(const f of readdirSync(root+"/"+dir)){
  if(!/^[a-f0-9]{64}\.json$/.test(f)){errs.set("name:"+f.slice(0,20),(errs.get("name:"+f.slice(0,20))||0)+1);continue}
  const b=readFileSync(root+"/"+dir+"/"+f);
  if(createHash("sha256").update(b).digest("hex")!==f.slice(0,-5)){badSha++;}
  try{fn(JSON.parse(b.toString("utf8")),W);ok++}catch(e){bad++;const k=String(e.message).slice(0,80);errs.set(k,(errs.get(k)||0)+1)}
 }
}
console.log(JSON.stringify({ok,bad,badSha,ms:Math.round(performance.now()-t0),errs:[...errs]}));
