import {join} from "node:path";
import {casEnforceTickV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🚦️guard/🟦️.ts";
import {CAS_POLICY_V1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧬️schema/🟦️.ts";
const root=join(process.argv[2]!,"cas");const policy={...CAS_POLICY_V1,capBytes:1<<20,minimumCapBytes:1,minAgeMs:3600*1000};
const gaps:number[]=[];let last=performance.now();const timer=setInterval(()=>{const at=performance.now();gaps.push(at-last-5);last=at;},5);
for(let i=0;i<2;i++){const t=performance.now();gaps.length=0;await casEnforceTickV1({root,policy});const s=gaps.slice().sort((a,b)=>a-b);console.log("tick",i,(performance.now()-t).toFixed(0),"ms; lag p99",s[Math.floor(s.length*.99)]?.toFixed(1),"max",s.at(-1)?.toFixed(0),"over100",s.filter(x=>x>100).length);}
process.exit(0);
