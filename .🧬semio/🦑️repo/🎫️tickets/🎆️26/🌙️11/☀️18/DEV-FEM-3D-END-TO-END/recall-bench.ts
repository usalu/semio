import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {recallAdmissionV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";
const root="C:/semio/.cache/b/r/m/u",a=root+"/a";let hit=0,n=0,bytes=0;const t=performance.now();
for(const d of readdirSync(a))for(const f of readdirSync(join(a,d)))if(f.endsWith(".json")){const m=JSON.parse(readFileSync(join(a,d,f),"utf8"));n++;const r=recallAdmissionV1(root,m.key);if(r){hit++;bytes+=r.inputs.reduce((s:number,i:any)=>s+i.bytes,0);}}
console.log({units:n,recalled:hit,sourceMBNotReread:+(bytes/1e6).toFixed(1),ms:Math.round(performance.now()-t)});
