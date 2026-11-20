import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {stampPathV1,RACY_NANOSECONDS_V1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";
const root="C:/semio/.cache/b/r/m/u/a";let units=0,racy=0,absent=0,noIno=0;const ages:number[]=[];
const horizon=BigInt(Date.now())*1_000_000n-RACY_NANOSECONDS_V1;
for(const d of readdirSync(root))for(const f of readdirSync(join(root,d)))if(f.endsWith(".json")){const m=JSON.parse(readFileSync(join(root,d,f),"utf8"));units++;let bad=false;for(const s of m.stamps){const n=stampPathV1(s.path);if(!n.present){absent++;continue;}if(n.ino===""){noIno++;bad=true;continue;}const a=Number((BigInt(Date.now())*1_000_000n-BigInt(n.ctimeNs))/1_000_000n);ages.push(a);if(BigInt(n.ctimeNs)>=horizon||BigInt(n.mtimeNs)>=horizon)bad=true;}if(bad)racy++;}
ages.sort((a,b)=>a-b);console.log({units,wouldRefuseNow:racy,absentStamps:absent,noIno,youngestCtimeAgeMs:ages[0],p1:ages[Math.floor(ages.length*.01)],median:ages[ages.length>>1]});
import {rememberAdmissionV1,recallAdmissionV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📖️admission/🟦️.ts";
const scratch="C:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-FEM-3D-END-TO-END/🗑️generated/admission-bench/remember";let ok=0,no=0,stale=0,staleOk=0;
for(const d of readdirSync(root))for(const f of readdirSync(join(root,d)))if(f.endsWith(".json")){const m=JSON.parse(readFileSync(join(root,d,f),"utf8")),fresh=!!recallAdmissionV1("C:/semio/.cache/b/r/m/u",m.key);const r=rememberAdmissionV1(scratch,m.key,{inputs:m.inputs,cfg:m.cfg,environment:m.environment,probed:m.stamps.map((s:any)=>s.path),startedMs:Date.now()-5000});if(r)ok++;else no++;if(!fresh){stale++;if(r)staleOk++;}}
console.log({rememberOk:ok,refused:no,staleUnits:stale,staleRememberOk:staleOk});
