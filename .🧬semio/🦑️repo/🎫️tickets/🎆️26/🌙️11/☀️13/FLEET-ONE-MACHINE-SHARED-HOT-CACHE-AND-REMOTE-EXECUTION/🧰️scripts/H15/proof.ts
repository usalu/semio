import {readFileSync,readdirSync} from "node:fs";import {createHash} from "node:crypto";
import {canonicalDepInfoV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🔗️sources/🟦️.ts";
const EV="/Users/ueli/Documents/semio/.cache/b/p/m/fleet/cache-server/fleet/evidence/";const h=(t:string)=>createHash("sha256").update(t).digest("hex").slice(0,10);
let before=0,after=0,total=0;
for(const id of readdirSync(EV).filter(i=>i.includes("-unit-"))){const m=JSON.parse(readFileSync(EV+id+"/meta.json","utf8"));
 for(const n of m.differing.filter((n:string)=>n.endsWith(".d"))){const f=(s:string)=>m.files.find((x:any)=>x.path===n&&x.side===s).name;
  const a=readFileSync(EV+id+"/"+f("kept"),"utf8"),b=readFileSync(EV+id+"/"+f("refused"),"utf8");total++;if(a!==b)before++;
  const same=canonicalDepInfoV1(a)===canonicalDepInfoV1(b);if(!same)after++;console.log(id.slice(14,26),n.split("/").pop()!.slice(0,40),h(a),h(b),"->",h(canonicalDepInfoV1(a)),h(canonicalDepInfoV1(b)),same?"IDENTICAL":"STILL DIFFERENT");}}
console.log({total,differingBefore:before,differingAfter:after});
